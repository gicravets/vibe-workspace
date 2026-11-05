"""Candidate contract checks, not runtime authorization or upstream Rust tests."""
import copy
import json
from pathlib import Path
import unittest

from jsonschema import Draft202012Validator


HOME = Path(__file__).resolve().parents[1] / 'contracts' / 'fup-workspace'


class WorkspaceContracts(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.profile = json.loads((HOME / 'profile.example.json').read_text())
        cls.profile_schema = json.loads((HOME / 'profile.schema.json').read_text())
        cls.obs_schema = json.loads((HOME / 'observation-request.schema.json').read_text())
        cls.pv = Draft202012Validator(cls.profile_schema)
        cls.ov = Draft202012Validator(cls.obs_schema)

    def test_schemas_are_valid(self):
        Draft202012Validator.check_schema(self.profile_schema)
        Draft202012Validator.check_schema(self.obs_schema)

    def test_candidate_fixture(self):
        self.pv.validate(self.profile)

    def test_all_clients_are_design_candidates(self):
        for client in ('codex', 'openclaw', 'claude'):
            profile = copy.deepcopy(self.profile)
            profile['client'] = client
            self.pv.validate(profile)

    def test_profile_cannot_activate_execution(self):
        profile = copy.deepcopy(self.profile)
        profile['execution_enabled'] = True
        self.assertFalse(self.pv.is_valid(profile))

    def test_profile_does_not_carry_identity_credentials_or_commands(self):
        for key in ('thread_uid', 'token', 'grants', 'command', 'args', 'root_path'):
            profile = copy.deepcopy(self.profile)
            profile[key] = 'not accepted'
            self.assertFalse(self.pv.is_valid(profile), key)

    def test_requirements_cannot_be_disabled(self):
        for key in self.profile['requirements']:
            profile = copy.deepcopy(self.profile)
            profile['requirements'][key] = False
            self.assertFalse(self.pv.is_valid(profile), key)

    def test_pins_need_revision_hash_and_entity_type(self):
        for key in ('revision', 'sha256', 'entity_type'):
            profile = copy.deepcopy(self.profile)
            del profile['knowledge_pins'][0][key]
            self.assertFalse(self.pv.is_valid(profile), key)

    def test_qualified_pin_and_hash_shape(self):
        for key, value in (('project_uid', 'same-name'), ('record_uid', 'skill.md'), ('revision', 0), ('sha256', 'HEAD')):
            profile = copy.deepcopy(self.profile)
            profile['knowledge_pins'][0][key] = value
            self.assertFalse(self.pv.is_valid(profile), key)

    def request(self):
        return {'project_uid': self.profile['project_uid'], 'request_key': 'fixture-v1',
                'title': 'Fixture', 'question': 'What happened?', 'observed': 'Not a real event',
                'related_record_uids': ['KIT-TASK-000001'], 'assessment': 'partial'}

    def test_existing_capture_argument_shape(self):
        self.ov.validate(self.request())

    def test_caller_or_approval_cannot_be_in_capture_payload(self):
        for key in ('thread_uid', 'client', 'subject', 'status', 'dataset_split', 'approved_by', 'credential'):
            request = self.request()
            request[key] = 'not accepted'
            self.assertFalse(self.ov.is_valid(request), key)

    def test_doc_refs_are_not_journal_refs(self):
        request = self.request()
        request['related_record_uids'] = ['KIT-DOC-000001']
        self.assertFalse(self.ov.is_valid(request))

    def test_blank_content_and_invalid_key(self):
        for key, value in (('request_key', ' key '), ('request_key', ''), ('title', ' '), ('question', '\n'), ('observed', '')):
            request = self.request()
            request[key] = value
            self.assertFalse(self.ov.is_valid(request), key)

    def test_capture_bounds_and_duplicate_refs(self):
        for key, value in (('title', 'a' * 301), ('question', 'a' * 4001),
                           ('observed', 'a' * 12001),
                           ('related_record_uids', ['KIT-TASK-000001'] * 2),
                           ('related_record_uids', ['KIT-TASK-%06d' % i for i in range(21)])):
            request = self.request()
            request[key] = value
            self.assertFalse(self.ov.is_valid(request), key)


if __name__ == '__main__':
    unittest.main()
