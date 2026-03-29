import pytest
import sprocket_py

def test_wdl_parsing():
    source = "version 1.0\ntask hello {}"
    doc = sprocket_py.parse_wdl(source)
    
    assert doc.version == "1.0"
    assert len(doc.warnings) == 0

def test_empty_source_warning():
    doc = sprocket_py.parse_wdl("task incomplete {}")
    assert any("Missing WDL version" in w.message for w in doc.warnings)

def test_invalid_input():
    with pytest.raises(Exception):
        sprocket_py.parse_wdl("")