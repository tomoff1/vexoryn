# Publier VEXORYN gratuitement

Le dépôt et les workflows sont préparés localement. Aucun dépôt distant, release ou site public n'est créé sans compte GitHub et destination réels.

La destination choisie est **tomoff1/vexoryn**. Le site et le centre de mises à jour utilisent cette destination. Cela ne signifie pas que le dépôt ou une release existent déjà.

## Préparer un dépôt de marque

1. Connectez-vous à GitHub. Pour respecter la confidentialité, utilisez un compte ou une organisation portant uniquement la marque VEXORYN. Les noms d'auteur des commits, profils GitHub et chemins de dépôt sont publics.
2. Créez un dépôt public, par exemple `vexoryn`, sans initialiser README ni licence puisque ces fichiers existent déjà.
3. Avant votre premier commit, configurez une identité de marque et une adresse GitHub `noreply` associée à ce compte. N'utilisez aucune coordonnée personnelle dans les sources ou métadonnées publiques.
4. Depuis le dossier du projet, utilisez les commandes ci-dessous en remplaçant `COMPTE_DE_MARQUE` par votre véritable compte. Ce placeholder n'est pas une URL de publication existante.

```powershell
git init
git branch -M main
git add .
git commit -m "VEXORYN initial release"
git remote add origin https://github.com/tomoff1/vexoryn.git
git push -u origin main
```

Vérifiez `.gitignore` avant `git add` : aucun fichier de configuration personnelle, journal, cache, dépendance ou installateur ne doit être commité. Git peut demander votre authentification.

Dans le dossier de travail déjà préparé, `origin` pointe sur `tomoff1/vexoryn` : il est inutile de refaire `git remote add`. Si vous repartez du ZIP des sources, initialisez le dépôt et ajoutez l'adresse comme dans les commandes ci-dessus. Si un autre `origin` existe, remplacez `add` par `set-url`.

## Compiler dans GitHub Actions

L'onglet **Actions → Windows checks and installers** exécute les tests front-end et Rust, compile `.exe` et `.msi`, puis fournit l'artefact `VEXORYN-Windows-x64` et ses SHA-256. Un workflow préparé n'est pas un workflow déjà exécuté ; vérifiez son état vert et téléchargez ses artefacts.

Le build utilise des outils tiers téléchargés par Tauri. Les versions exactes des bibliothèques sont fixées par `package-lock.json` et `Cargo.lock` ; conservez ces deux fichiers.

## Publier une version

1. Mettez la même version dans `package.json`, `src-tauri/Cargo.toml` et `src-tauri/tauri.conf.json`.
2. Faites vos tests et un commit, puis poussez sur `main`.
3. Créez un tag correspondant :

```powershell
git tag v0.1.0
git push origin v0.1.0
```

4. Après compilation réussie, le workflow crée une **release brouillon**. Dans **Releases**, contrôlez ses deux installateurs, `SHA256SUMS.txt` et ses notes.
5. Installez et désinstallez les bundles sur une machine de test Windows. Testez les opérations sur une console autorisée et consignez les résultats. Présentez les limites restantes dans les notes.
6. Cliquez **Publish release** quand ces contrôles sont faits. Le site ne propose que la dernière release publique stable ; une release brouillon ou prerelease n'est pas présentée comme stable.

Le SHA-256 détecte une différence entre fichiers mais ne remplace pas la signature Authenticode ni l'authentification de l'éditeur. La mise à jour automatique Tauri signée n'est pas activée : elle exige des clés, un manifeste signé et une politique de distribution. La première version vérifie les releases et laisse l'installation à l'utilisateur.

## Activer le site GitHub Pages

1. Dans le dépôt, ouvrez **Settings → Pages**.
2. Choisissez **GitHub Actions** comme source.
3. Ouvrez **Actions → VEXORYN website → Run workflow**.
4. Une fois le déploiement réussi, GitHub indique l'adresse dans **Settings → Pages** et dans l'environnement `github-pages`.

Le workflow copie `website/` et renseigne le véritable `github.repository` dans `config.json`. Le site interroge l'API GitHub pour afficher les assets réels. Sans release ou sans configuration, il affiche un état explicite. Aucun VPS ni secret de téléchargement n'est nécessaire. Voir le [guide officiel GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages).

## Ajouter des captures réelles

Une capture réelle de VEXORYN 0.1.0 exécuté sur Windows est fournie dans `website/screenshots/vexoryn-dashboard.png`. Elle montre l'application hors connexion et sa légende le précise. Pour compléter la galerie, capturez le programme réellement exécuté, masquez les adresses et chemins privés, puis placez les images dans `website/screenshots/`. Ajoutez une légende précisant version et date.
