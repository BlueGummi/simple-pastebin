let fileName;
let previousLogData = ''; // To track previous log data

// Theme changer
let checkbox = document.getElementById('checkbox');
let body = document.body;
let icon = document.getElementById('icon');
let toggle = document.getElementById('toggle');

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

// Load the expiration value and log name from config.toml
async function loadConfig() {
    try {
        let response = await fetch('config.toml');
        if (!response.ok) throw new Error('Network response was not ok');
        let text = await response.text();
        let expirationValue = parseExpiration(text);
        displayExpirationMessage(expirationValue);
        fileName = parseFileName(text);
    } catch (error) {
        document.getElementById('expirationMessage').textContent = 'Error loading expiration';
    }
}

function parseFileName(tomlText) {
    let logNameMatch = tomlText.match(/log_name\s*=\s*"(.*?)"/);
    if (logNameMatch) {
        let logName = removeQuotes(logNameMatch[1]);
        if (logName.startsWith('#')) {
            return 'input.log';
        }
        return logName;
    }
    return 'input.log';
}

function removeQuotes(text) {
    return text.replace(/^"|"$/g, '');
}

function parseExpiration(tomlText) {
    let expirationMatch = tomlText.match(/expiration\s*=\s*"(.*?)"/);
    return expirationMatch ? expirationMatch[1] : '10m';
}

function displayExpirationMessage(expirationValue) {
    let timeUnits = { d: 'day', h: 'hour', m: 'minute', s: 'second' };
    let parts = expirationValue.match(/(\d+)([dhms])/g) || [];
    let formattedParts = parts.map(part => {
        let value = part.slice(0, -1);
        let unit = part.slice(-1);
        return `${value} ${timeUnits[unit] + (value > 1 ? 's' : '')}`;
    });
    document.getElementById('expirationMessage').textContent = ` (Clears every ${formattedParts.join(', ')})`;
}

// Load log with change detection
async function loadLog() {
    try {
        let response = await fetch(fileName);
        if (!response.ok) throw new Error('Network response was not ok');
        let text = await response.text();
        
        // Check if the log has changed
        if (text !== previousLogData) {
            previousLogData = text; // Update previous log data
            parseLogData(text); // Only parse if there is a change
        }
    } catch (error) {
        document.getElementById('fileContent').textContent = 'Error loading log';
        document.getElementById('timestamps').textContent = '';
    }
}

function parseLogData(data) {
    let lines = data.split('\n');
    let timestamps = [];
    let content = [];
    let timestampRegex = /^\d{2}\/\d{2}\/\d{2} \d{2}:\d{2}:\d{2} (AM|PM) \|: /;

    lines.forEach(line => {
        let match = timestampRegex.exec(line);
        if (match) {
            let timestamp = line.substring(0, match[0].length - 3).trim();
            let contentText = line.substring(match[0].length).trim();

            // Convert URLs in contentText to <a> tags
            contentText = convertUrlsToLinks(contentText);

            timestamps.push(timestamp);
            content.push(contentText);
        } else {
            timestamps.push('');
            content.push(convertUrlsToLinks(line));
        }
    });

    // Display timestamps and content
    document.getElementById('timestamps').textContent = timestamps.join('\n');
    document.getElementById('fileContent').innerHTML = content.join('<br>'); // Use innerHTML to render <a> tags
}

// Function to convert URLs in text to anchor tags
function convertUrlsToLinks(text) {
    return text.replace(/(https?:\/\/[^\s]+)/g, url => 
        `<a href="${url}" target="_blank" rel="noopener noreferrer">${url}</a>`
    );
}

function downloadLog() {
    fetch(fileName)
        .then(response => response.blob())
        .then(blob => {
            let url = URL.createObjectURL(blob);
            let a = document.createElement('a');
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
            let response = await fetch('/clear', { method: 'POST' });
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

document.addEventListener('DOMContentLoaded', () => {
    console.log("Thing load");
    document.getElementById('inputForm').addEventListener('submit', async function(event) {
        event.preventDefault();
        const inputData = document.getElementById('input').value;
        console.log('Form submitted with data:', inputData);

        try {
            const response = await fetch('/', {
                method: 'POST',
                headers: {
                    'Content-Type': 'text/plain',
                },
                body: inputData
            });

            console.log('Response status:', response.status);

            if (response.ok) {
                const text = await response.text();
                console.log('Server response:', text);
                loadLog();
            } else {
                console.error('Error:', response.statusText);
            }
        } catch (error) {
            console.error('Fetch error:', error);
        }

        document.getElementById('input').value = '';
    });
});

// Load configuration and initial log on window load
window.onload = () => {
    loadConfig(); // Load the configuration
    loadLog(); // Load the log
};
setInterval(loadLog, 500);