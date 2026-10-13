# État de la première version

Ce document distingue le code livré de sa validation. Les rapports effectifs de compilation et tests de la session sont à consulter avant diffusion. Aucune console PS5 réelle n'a été mise à disposition : **toutes les fonctions console restent à valider sur matériel**.

## Périmètre de la version technique

- Interface Windows Tauri / React avec navigation, français / anglais et thèmes sombres.
- Connexion manuelle à une adresse privée et diagnostics des ports configurés.
- Explorateur FTP, transferts et commandes fichiers selon le service serveur.
- Envoi de fichiers ELF locaux par TCP vers un chargeur compatible.
- Catalogue de références communautaires sans binaires tiers intégrés.
- Analyse d'erreurs cheats à partir des informations fournies, sans promettre de moteur ou de cheats compatibles.
- Journaux locaux et paramètres ; disponibilité des données console explicitement limitée.
- Pipeline de compilation Windows NSIS / MSI, artefacts et SHA-256.
- Site statique GitHub Pages, téléchargement des assets d'une release réellement publiée.

La compilation Windows et le déploiement Pages sur GitHub Actions ont réussi ; voir le rapport de validation. La présence d'une commande réseau ne garantit pas une intégration compatible avec toutes les variantes de serveurs.

## Limites actuelles et prochaines étapes

| Sujet | État / travail nécessaire |
| --- | --- |
| Firmware, température, espace disque | API documentée et vérifiée à intégrer ; ne pas fabriquer ces données. |
| Identification automatique PS5 | Des services TCP sont seulement des candidats. Ajouter mDNS ou une API d'identité explicite. |
| PKG | Transfert comme fichier FTP possible. Inspection du format, compatibilité et installation distante avec accusé console restent à intégrer. |
| Homebrews installés | Inventaire et gestion nécessitent un adaptateur réel. |
| Cheats | Pas de téléchargement de catalogue, d'installation moteur ni d'activation automatique dans cette version. |
| Reprise FTP / annulation | Pas de reprise automatique REST ni d'annulation du transport dans cette version ; à développer puis vérifier par serveur. |
| Intégrité distante | Nécessite hash serveur ou téléchargement de contrôle ; la taille seule ne prouve pas le SHA-256. |
| Sauvegarde / restauration | Des fichiers peuvent être transférés ; une sauvegarde console cohérente et restauration transactionnelle restent à développer. |
| Captures | Transfert manuel via fichiers ; inventaire spécialisé à développer. |
| Plugins | Catalogue / protocole / signature et contrôle de compatibilité à définir. |
| Mises à jour | Consultation des releases ; application automatique signée à développer. |
| Signature Windows | Certificat Authenticode à fournir avant diffusion de confiance. |
| Publication | Dépôt public, release 0.1.0 et site Pages publiés ; compilation Windows CI réussie. |
| PS5 native | Étude SDK uniquement ; aucun ELF ou PKG VEXORYN natif produit. |

## Ordre conseillé

1. Tests FTP et chargeur ELF sur une vraie console ; consigner chaque serveur et version.
2. Fiabiliser annulation, reprise, vérification distante, restauration et files d'attente persistantes.
3. Ajouter un adaptateur en lecture seule pour identité, métriques et versions de jeux.
4. Intégrer un service PKG documenté, avec statut final confirmé par la console.
5. Installer des cheats par moteur avec sauvegarde vérifiée et gestion des conflits.
6. Signer les installateurs et activer un updater signé.
7. Compiler et tester le homebrew natif avant tout package public.
