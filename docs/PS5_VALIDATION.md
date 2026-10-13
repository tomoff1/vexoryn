# Validation sur une console réelle

État initial : **non effectué**. Ce document est une procédure de recette, pas un rapport de réussite. Remplacez « non testé » par un résultat uniquement après observation.

## Contexte à enregistrer

Version VEXORYN, Windows, firmware console lu sur la console, version HEN, version FTP, type/version du chargeur, ports, connexion Ethernet/Wi-Fi. Dans un rapport public, retirez les adresses IP, identifiants, chemins personnels et coordonnées. Conservez uniquement les informations techniques utiles.

| Test | Résultat initial | Observation à consigner |
| --- | --- | --- |
| Connexion vers une IP locale autorisée | Non testé | Temps, service, messages d'erreur |
| Port fermé / timeout / IP invalide | Non testé | Erreur claire, interface réactive |
| Listing dossier FTP vide et rempli | Non testé | Noms, Unicode, tailles, dossiers |
| Envoi et téléchargement fichier 1 Ko puis 100 Mo | Non testé | Taille et SHA-256 après aller-retour |
| Création dossier, remplacement avec sauvegarde, suppression confirmée | Non testé | Résultat console, sauvegarde visible, confirmation préalable |
| Coupure réseau au milieu d'un transfert | Non testé | Aucun faux succès ; fichier partiel signalé |
| Annulation / fermeture de l'application | Non testé | Connexions libérées, journal cohérent |
| Envoi d'un ELF autorisé sans danger | Non testé | Chargeur compatible ; exécution vérifiée séparément |
| ELF invalide / format BIN / mauvais chargeur | Non testé | Rejet ou erreur claire ; pas de faux succès |
| Diagnostics cheats version manquante | Non testé | Version inconnue explicitement, hypothèses distinctes |
| Installation et désinstallation Windows | Non testé | Raccourci, lancement, suppression du logiciel |

Utilisez un dossier console temporaire dédié pour les opérations fichiers. Ne testez pas des suppressions dans les dossiers système. Sauvegardez les fichiers existants avant remplacement. N'envoyez aucun payload dont la source, la compatibilité et la fonction n'ont pas été vérifiées.
