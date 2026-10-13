"use strict";
async function loadRelease() {
  const status = document.getElementById("release-status");
  try {
    const configResponse = await fetch("./config.json", {cache:"no-store"});
    if (!configResponse.ok) throw new Error("Configuration indisponible");
    const {repository} = await configResponse.json();
    if (!repository) return;
    if (!/^[A-Za-z0-9][A-Za-z0-9_.-]*\/[A-Za-z0-9][A-Za-z0-9_.-]*$/.test(repository)) throw new Error("Dépôt invalide");
    document.getElementById("release-title").textContent = "Publication VEXORYN";
    const base = `https://github.com/${repository}`;
    const issues = document.getElementById("issues");
    issues.href = `${base}/issues`; issues.classList.remove("hidden");
    document.getElementById("support-text").textContent = "Joignez la version de VEXORYN, le service utilisé et un journal dont vous avez retiré les données personnelles.";
    const response = await fetch(`https://api.github.com/repos/${repository}/releases/latest`, {headers:{Accept:"application/vnd.github+json"}});
    if (response.status === 404) { status.textContent = "Le dépôt est configuré. Aucune version publique stable n'est encore disponible."; return; }
    if (!response.ok) throw new Error(`GitHub : HTTP ${response.status}`);
    const release = await response.json();
    const validUrl = (url) => { const u = new URL(url); return u.protocol === "https:" && u.hostname === "github.com" && u.pathname.startsWith(`/${repository}/releases/`); };
    const assets = release.assets.filter(a => /\.(exe|msi)$|^SHA256SUMS\.txt$/.test(a.name) && validUrl(a.browser_download_url));
    document.getElementById("release-title").textContent = `VEXORYN ${release.tag_name}`;
    status.textContent = assets.length ? "Installateurs issus de la dernière release GitHub. Vérifiez les sommes SHA-256 avant installation." : "Cette release ne contient pas encore d'installateur Windows.";
    for (const asset of assets) { const a = document.createElement("a"); a.className = "button"; a.href = asset.browser_download_url; a.textContent = asset.name; document.getElementById("assets").append(a); }
    const installer = assets.find(a => a.name.endsWith(".exe"));
    if (installer) { const a = document.getElementById("download"); a.href = installer.browser_download_url; a.textContent = "Télécharger pour Windows ↗"; a.classList.remove("disabled"); a.removeAttribute("aria-disabled"); }
    if (validUrl(release.html_url)) { const a = document.createElement("a"); a.className = "secondary"; a.href = release.html_url; a.textContent = "Notes complètes sur GitHub ↗"; document.getElementById("assets").append(a); }
  } catch (error) { status.textContent = `Vérification des releases indisponible : ${error.message}. Réessayez plus tard.`; }
}
loadRelease();

