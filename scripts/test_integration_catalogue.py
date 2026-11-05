"""Static source registration review; does not execute MCP or Git operations."""
import json,re,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
class Catalogue(unittest.TestCase):
 def test_matches_actual_registered_handlers(self):
  catalogue=json.loads((ROOT/'integration/tool-catalogue.json').read_text())
  registered=re.findall(r'with_tool\(Arc::new\(handlers::(\w+)\)\)',(ROOT/'src/mcp/server.rs').read_text())
  handlers={}
  for path in (ROOT/'src/mcp/handlers').glob('*.rs'):
   for typ,name in re.findall(r'impl VibeToolHandler for (\w+)\s*\{\s*fn tool_name\(&self\) -> &str\s*\{\s*"([^"]+)"',path.read_text()):handlers[typ]=name
  self.assertEqual([handlers[x]for x in registered],[x['name']for x in catalogue['tools']])
  self.assertEqual(len(registered),len(set(registered)))
 def test_source_catalogue_cannot_claim_admission(self):
  catalogue=json.loads((ROOT/'integration/tool-catalogue.json').read_text())
  self.assertEqual(catalogue['state'],'source_review_not_execution_admission')
  for row in catalogue['tools']:
   self.assertIs(row['available_to_shared_gateway'],False)
   self.assertIs(row['native_permission_verified'],False)
if __name__=='__main__':unittest.main()
