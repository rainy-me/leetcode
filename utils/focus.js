window.addEventListener("click", () => {
  document.querySelectorAll("tr").forEach((tr) => {
    let lock = tr.querySelector('[data-original-title="Subscribe to unlock"]');
    let ok = tr.querySelector(".text-success");
    if (lock || ok) {
      tr.style.opacity = 0.1;
    }
  });
});
