let previousLogData = ''; 

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

async function loadConfig() {
    try {
        let response = await fetch('/config');
        if (!response.ok) throw new Error('Network response was not ok');
        let text = await response.text();
        let lines = text.split('\n');
        let expirationValue = lines[0].trim(); 
        displayExpirationMessage(expirationValue);
    } catch (error) {
        document.getElementById('expirationMessage').textContent = 'Error loading expiration';
        console.error('Error loading config:', error);
    }
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

async function loadLog() {
    try {
        await loadConfig(); 

        let response = await fetch('/log');

        if (!response.ok) throw new Error('Network response was not ok');

        let text = await response.text();

        if (text !== previousLogData) {
            previousLogData = text; 
            parseLogData(text); 
        }
    } catch (error) {
        document.getElementById('fileContent').textContent = 'Error loading log';
        document.getElementById('timestamps').textContent = '';
        console.error('Error loading log:', error);
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

            contentText = convertUrlsToLinks(contentText);

            timestamps.push(timestamp);
            content.push(contentText);
        } else {
            timestamps.push('');
            content.push(convertUrlsToLinks(line));
        }
    });

    document.getElementById('timestamps').textContent = timestamps.join('\n');
    document.getElementById('fileContent').innerHTML = content.join('<br>'); 
}

function convertUrlsToLinks(text) {
    return text.replace(/(https?:\/\/[^\s]+)/g, url => 
        `<a href="${url}" target="_blank" rel="noopener noreferrer">${url}</a>`
    );
}

function downloadLog() {
    fetch('/log')
        .then(response => response.blob())
        .then(blob => {
            let url = URL.createObjectURL(blob);
            let a = document.createElement('a');
            a.href = url;
            a.download = 'log'; 
            document.body.appendChild(a);
            a.click();
            a.remove();
            URL.revokeObjectURL(url);
        })
        .catch(error => console.error('Error downloading log:', error));
}

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
async function createNewPaste(inputData) {
    console.log("Creating new paste with data:", inputData);
    try {
        let response = await fetch('/new', {
            method: 'POST',
            headers: {
                'Content-Type': 'text/plain',
            },
            body: inputData
        });

        console.log("Response received:", response);
        if (!response.ok) throw new Error('Network response was not ok');

        let message = await response.text();
        const numberMatch = message.match(/\/\d+$/);

        if (numberMatch) {
            const number = numberMatch[0];
            let clientUrl = `${window.location.origin}${number}`;
            clientUrl = convertUrlsToLinks(clientUrl);
            message = `Paste successful, view it at ${clientUrl}`;
        }

        document.getElementById('newPasteMessage').innerHTML = message;
        loadLog();
    } catch (error) {
        console.error('Error creating new paste:', error);
        document.getElementById('newPasteMessage').textContent = 'Error creating new paste.';
    }
}
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

        if (response.ok) {
            const text = await response.text();
            document.getElementById('newPasteMessage').textContent = text; 
            loadLog(); 
        } else {
            console.error('Error:', response.statusText);
            document.getElementById('newPasteMessage').textContent = 'Error submitting form.';
        }
    } catch (error) {
        console.error('Fetch error:', error);
        document.getElementById('newPasteMessage').textContent = 'Error submitting form.';
    }

    document.getElementById('input').value = ''; 
});

document.getElementById('newPasteButton').addEventListener('click', async () => {
    const inputData = document.getElementById('input').value; 
    if (inputData.trim() === "") {
        document.getElementById('newPasteMessage').textContent = 'Please enter some text.';
        return;
    }
    await createNewPaste(inputData); 
    document.getElementById('input').value = ''; 
});

document.getElementById('deleteButton').addEventListener('click', deleteLog);
document.getElementById('downloadButton').addEventListener('click', downloadLog); 

function autoResize(textarea) {
    textarea.style.height = 'auto';
    textarea.style.height = `${textarea.scrollHeight}px`;
}

window.onload = () => {
    loadConfig(); 
    loadLog(); 
};
setInterval(loadLog, 500);
