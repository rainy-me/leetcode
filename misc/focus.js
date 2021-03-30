const focus = () => {
  document.querySelectorAll("tr").forEach((tr) => {
    let lock = tr.querySelector('[data-original-title="Subscribe to unlock"]');
    let done = tr.querySelector(".text-success");
    if (lock || done) {
      tr.style.opacity = 0.1;
    }
  });
};
window.addEventListener("click", focus);
setTimeout(focus, 1000);
