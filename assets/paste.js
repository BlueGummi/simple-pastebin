let checkbox = document.getElementById('checkbox');
let body = document.body;
let icon = document.getElementById('icon');
let toggle = document.getElementById('toggle');
document.getElementById('checkbox').checked = false;

checkbox.addEventListener('change', () => {
    if (checkbox.checked) {
        body.classList.remove('light');
        body.classList.add('dark');
        icon.textContent = '🌙';
        toggle.classList.add('active');
    } else {
        body.classList.remove('dark');
        body.classList.add('light');
        icon.textContent = '☀️';
        toggle.classList.remove('active');
    }
});
