#[cfg(test)]
mod ui_tests {
    
    use super::*;
    use crate::ui::{render_ui, UiElement};

    #[test]
    fn test_ui_rendering() {
        // Prépare des éléments d'interface utilisateur fictifs pour le test
        let ui_elements = vec![
            UiElement::new("Button", (100, 100)),
            UiElement::new("TextBox", (200, 200)),
           
        ];

        
        let rendered_ui = render_ui(&ui_elements);

        // Vérifie que le rendu de l'interface utilisateur est conforme aux attentes
        assert_eq!(rendered_ui.len(), ui_elements.len());
        
    }
}
