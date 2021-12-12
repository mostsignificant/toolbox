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
        url: 'http://ip-api.com/json',
        async: false
    }).responseText;

    try {
        var json = JSON.parse(data);
        return json["query"];
    } catch (e) {
        return "";
    }
}
