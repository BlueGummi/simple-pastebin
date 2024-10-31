let checkbox = document.getElementById('checkbox');
let body = document.body;
let icon = document.getElementById('icon');
let toggle = document.getElementById('toggle');
checkbox.checked = false;
function toggleTheme() {
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
}
checkbox.addEventListener('change', toggleTheme);
const deleteButton = document.getElementById('deleteButton');
deleteButton.addEventListener('click', async (event) => {
    event.preventDefault();
    if (!confirm('Are you sure you want to delete this paste?')) {
        return;
    }
    const pasteId = getPasteIdFromUrl();
    if (pasteId) {
        try {
            const response = await fetch(`/${pasteId}/delete`, {
                method: 'POST',
            });
            if (response.ok) {
                const message = await response.text();
                alert(message);
                location.reload();
            } else {
                alert('Failed to delete paste.');
            }
        } catch (error) {
            console.error('Error:', error);
            alert('An error occurred while deleting the paste.');
        }
    } else {
        alert('Invalid paste ID.');
    }
});

function getPasteIdFromUrl() {
    const urlParts = window.location.pathname.split('/');
    return urlParts[urlParts.length - 1];
}