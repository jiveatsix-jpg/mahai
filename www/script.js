const invoke = window.__TAURI__.core.invoke;

const footerText = document.getElementById('footer-text');
const cards = document.querySelectorAll('.card');

cards.forEach((card) => {
  const statusEl = card.querySelector('[data-status]');
  card.addEventListener('click', async () => {
    const exeName = card.dataset.exe;
    const label = card.dataset.label;
    statusEl.textContent = 'LANZANDO...';
    footerText.textContent = `LANZANDO ${label.toUpperCase()}...`;
    try {
      await invoke('launch_app', { exeName });
      statusEl.textContent = 'EN EJECUCIÓN';
      footerText.textContent = `${label.toUpperCase()} INICIADO`;
    } catch (err) {
      statusEl.textContent = 'ERROR';
      footerText.textContent = `ERROR: ${err}`;
    }
    setTimeout(() => {
      statusEl.textContent = '';
      footerText.textContent = 'SISTEMA LISTO';
    }, 4000);
  });
});
