// Simple fade-in animation on page load
document.addEventListener('DOMContentLoaded', (event) => {
    const mainContent = document.querySelector('.main-content');

    // Use a small timeout to ensure the CSS transition triggers correctly
    // after the initial render.
    setTimeout(() => {
        mainContent.classList.add('visible');
    }, 100);
});
