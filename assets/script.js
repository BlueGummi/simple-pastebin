let fileName;

// Theme changer
document.getElementById('themeToggle').addEventListener('click', () => {
    document.body.classList.toggle('dark');
});

// Load the expiration value and log name from config.toml
async function loadConfig() {
    try {
        const response = await fetch('config.toml');
        if (!response.ok) throw new Error('Network response was not ok');
        const text = await response.text();
        const expirationValue = parseExpiration(text);
        displayExpirationMessage(expirationValue);
        fileName = parseFileName(text);
    } catch (error) {
        document.getElementById('expirationMessage').textContent = 'Error loading expiration';
    }
}

function parseFileName(tomlText) {
    const logNameMatch = tomlText.match(/log_name\s*=\s*"(.*?)"/);
    return logNameMatch ? removeQuotes(logNameMatch[1]) : '';
}

function removeQuotes(text) {
    return text.replace(/^"|"$/g, '');
}

function parseExpiration(tomlText) {
    const expirationMatch = tomlText.match(/expiration\s*=\s*"(.*?)"/);
    return expirationMatch ? expirationMatch[1] : '';
}

function displayExpirationMessage(expirationValue) {
    const timeUnits = { d: 'day', h: 'hour', m: 'minute', s: 'second' };
    const parts = expirationValue.match(/(\d+)([dhms])/g) || [];
    const formattedParts = parts.map(part => {
        const value = part.slice(0, -1);
        const unit = part.slice(-1);
        return `${value} ${timeUnits[unit] + (value > 1 ? 's' : '')}`;
    });
    document.getElementById('expirationMessage').textContent = ` (Clears every ${formattedParts.join(', ')})`;
}

// Load log
async function loadLog() {
    try {
        const response = await fetch(fileName);
        if (!response.ok) throw new Error('Network response was not ok');
        const text = await response.text();
        parseLogData(text);
    } catch (error) {
        document.getElementById('fileContent').textContent = 'Error loading log';
        document.getElementById('timestamps').textContent = '';
    }
}

// Parse log data
function parseLogData(data) {
    const lines = data.split('\n');
    const timestamps = [];
    const content = [];
    const timestampRegex = /^\d{2}\/\d{2}\/\d{2} \d{2}:\d{2}:\d{2} (AM|PM) \|: /;
    lines.forEach(line => {
        const match = timestampRegex.exec(line);
        if (match) {
            const timestamp = line.substring(0, match[0].length - 3).trim();
            const contentText = line.substring(match[0].length).trim();
            timestamps.push(timestamp);
            content.push(contentText);
        } else {
            timestamps.push('');
            content.push(line);
        }
    });
    document.getElementById('timestamps').textContent = timestamps.join('\n');
    document.getElementById('fileContent').textContent = content.join('\n');
}

function downloadLog() {
    fetch(fileName)
        .then(response => response.blob())
        .then(blob => {
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = fileName; 
            document.body.appendChild(a);
            a.click();
            a.remove();
            URL.revokeObjectURL(url);
        })
        .catch(error => console.error('Error downloading log:', error));
}

// Delete button
async function deleteLog() {
    if (confirm("Are you sure you want to delete the whole pastebin?")) {
        try {
            const response = await fetch('/clear', { method: 'POST' });
            if (!response.ok) throw new Error('Network response was not ok');
            loadLog();
        } catch (error) {
            console.error('Error clearing log:', error);
        }
    }
}

document.getElementById('deleteButton').addEventListener('click', deleteLog);
document.getElementById('downloadButton').addEventListener('click', downloadLog); 

function autoResize(textarea) {
    textarea.style.height = 'auto';
    textarea.style.height = `${textarea.scrollHeight}px`;
}

// Network stuff
document.getElementById('inputForm').addEventListener('submit', async (event) => {
    event.preventDefault();
    const inputData = document.getElementById('input').value;
    if (inputData) {
        try {
            const response = await fetch('/', {
                method: 'POST',
                headers: { 'Content-Type': 'text/plain' },
                body: inputData
            });
            if (!response.ok) throw new Error('Network response was not ok');
            document.getElementById('input').value = '';
            autoResize(document.getElementById('input'));
            loadLog();
        } catch (error) {
            console.error('Error submitting data:', error);
        }
    }
});

document.getElementById('input').addEventListener('keydown', function(event) {
    if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        document.getElementById('inputForm').dispatchEvent(new Event('submit'));
    }
});
        const themeToggle = document.getElementById('themeToggle');
        const body = document.body;
        const icon = document.getElementById('icon');

        themeToggle.addEventListener('click', () => {
            if (body.classList.contains('light')) {
                body.classList.replace('light', 'dark');
                icon.src = 'moon-icon.png'; // Change to your moon icon path
            } else {
                body.classList.replace('dark', 'light');
                icon.src = 'sun-icon.png'; // Change to your sun icon path
            }
        });
window.onload = () => {
    loadConfig(); // Load the configuration
    loadLog(); // Load the log
};

setInterval(loadLog, 500);