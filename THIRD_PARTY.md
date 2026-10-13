# Licences et attributions

Le code original VEXORYN est sous licence MIT. Aucun code source, icône, payload, cheat ou ressource graphique de Sony, PS5Upload ou des projets homebrew référencés n'est intégré intentionnellement.

Les dépendances réelles sont listées dans `package-lock.json` et `src-tauri/Cargo.lock`. Une distribution publique doit conserver les mentions des licences de toutes les dépendances et de leurs transitives. La liste ci-dessous est une orientation ; elle ne remplace pas l'inventaire exact des versions résolues.

L'inventaire généré est fourni dans [THIRD_PARTY_LICENSES.txt](THIRD_PARTY_LICENSES.txt). Il rassemble les expressions de licences et les fichiers LICENSE/COPYING/NOTICE des dépendances Rust résolues pour Windows et des paquets npm installés. Les installateurs embarquent ce fichier et la licence VEXORYN. Après modification des dépendances, exécutez `npm run licenses` avec Cargo disponible avant de reconstruire. Certaines dépendances sont utilisées uniquement à la compilation ; elles sont néanmoins conservées dans cet inventaire.

| Dépendance | Projet primaire | Licence amont |
| --- | --- | --- |
| React | https://github.com/facebook/react | MIT |
| TypeScript | https://github.com/microsoft/TypeScript | Apache-2.0 |
| Vite | https://github.com/vitejs/vite | MIT |
| Tauri | https://github.com/tauri-apps/tauri | MIT ou Apache-2.0 selon composant |
| Rust | https://www.rust-lang.org/policies/licenses | MIT / Apache-2.0 |

Les installateurs Tauri font appel à NSIS et WiX selon le format. Leur licence et les composants effectivement embarqués doivent être vérifiés dans les outils de packaging choisis. WebView2 est un runtime Microsoft distinct.

Les liens vers etaHEN, OnionHEN, ps5debug et les projets `ps5-payload-dev` renvoient à leurs auteurs. Ces projets conservent leurs propres licences. Ils ne sont ni redistribués ni présentés comme des composants VEXORYN. Une intégration de leur code ou de leurs binaires nécessite une revue de licence préalable, notamment pour les composants GPL.

Avant une release publique : exporter un inventaire de licences à partir des deux lockfiles, consulter les textes applicables aux versions choisies, placer les mentions requises dans les ressources distribuées, puis contrôler qu'aucune identité personnelle n'apparaît dans les métadonnées ou fichiers de publication.
