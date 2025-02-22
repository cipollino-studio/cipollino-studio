
use super::{EditorPanel, PANEL_KINDS};

impl serde::Serialize for EditorPanel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.panel.name())
    }
}

struct EditorPanelSerdeVisitor;

impl<'de> serde::de::Visitor<'de> for EditorPanelSerdeVisitor {
    type Value = EditorPanel;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a panel name")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error, {
        for panel_kind in PANEL_KINDS {
            if panel_kind.name == v {
                return Ok((panel_kind.make_panel)())
            }
        }
        Err(serde::de::Error::custom("unknown panel type"))
    } 
}

impl<'de> serde::Deserialize<'de> for EditorPanel {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        deserializer.deserialize_string(EditorPanelSerdeVisitor)
    }

}
