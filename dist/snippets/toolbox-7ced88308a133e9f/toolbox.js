export function copyToClipboard(id) {
    var element = document.getElementById(id);

    element.select();
    element.setSelectionRange(0, 99999); /* for mobile */

    navigator.clipboard.writeText(element.value);
}

export function getCurrentUTC() {
    return new Date().toUTCString();
}

export function getMyIP() {
    var data = $.ajax({
        url: 'https://api.db-ip.com/v2/free/self',
        async: false
    }).responseText;

    try {
        var json = JSON.parse(data);
        return json["ipAddress"];
    } catch (e) {
        return "";
    }
}
