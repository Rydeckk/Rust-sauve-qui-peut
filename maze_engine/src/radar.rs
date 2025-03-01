use commun::Deserialize;

/// Représente la vue radar autour du joueur dans le labyrinthe.
///
/// La structure `RadarView` contient trois champs qui décrivent l'état de la vue :
///
/// - `walls_horiz`: une matrice 4x3 de booléens indiquant l'état des murs horizontaux (chaque valeur est `true` si un mur est présent).
/// - `walls_vert`: une matrice 3x4 de booléens indiquant l'état des murs verticaux.
/// - `cells`: une matrice 3x3 de `u8` représentant les informations des cellules (par exemple, les indices ou la cible).
///
/// Ces informations sont généralement extraites d'une représentation binaire transmise par le serveur.
#[derive(Deserialize, Debug, PartialEq)]
pub struct RadarView {
    walls_horiz: [[bool; 3]; 4],
    walls_vert: [[bool; 4]; 3],
    pub cells: [[u8; 3]; 3],
}

impl RadarView {
    /// Crée une nouvelle instance de `RadarView` avec les matrices fournies.
    ///
    /// # Arguments
    ///
    /// * `walls_horiz` - Une matrice 4x3 de booléens représentant les murs horizontaux.
    /// * `walls_vert` - Une matrice 3x4 de booléens représentant les murs verticaux.
    /// * `cells` - Une matrice 3x3 de `u8` décrivant le contenu des cellules.
    ///
    /// # Exemple
    ///
    /// ```
    /// use maze_engine::radar::RadarView;
    ///
    /// let walls_horiz = [
    ///     [true, false, true],
    ///     [false, false, false],
    ///     [true, true, true],
    ///     [false, false, false],
    /// ];
    /// let walls_vert = [
    ///     [true, false, true, false],
    ///     [false, true, false, true],
    ///     [true, true, false, false],
    /// ];
    /// let cells = [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ];
    ///
    /// let radar_view = RadarView::new(walls_horiz, walls_vert, cells);
    /// assert_eq!(radar_view.cells[1][1], 4);
    /// ```
    pub fn new(walls_horiz: [[bool; 3]; 4], walls_vert: [[bool; 4]; 3], cells: [[u8; 3]; 3]) -> Self {
        Self {
            walls_horiz,
            walls_vert,
            cells,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Teste la fonction `new` de `RadarView` pour s'assurer que les matrices sont correctement assignées.
    #[test]
    fn test_new_radar_view() {
        let walls_horiz = [
            [true, false, true],
            [false, false, false],
            [true, true, true],
            [false, false, false],
        ];
        let walls_vert = [
            [true, false, true, false],
            [false, true, false, true],
            [true, true, false, false],
        ];
        let cells = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
        ];
        let radar_view = RadarView::new(walls_horiz, walls_vert, cells);
        assert_eq!(radar_view.walls_horiz, walls_horiz);
        assert_eq!(radar_view.walls_vert, walls_vert);
        assert_eq!(radar_view.cells, cells);
    }
}
