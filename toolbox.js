/**
 * Constants, functions, and key bindings for the toolbox site.
 */

/* CONSTANTS */

const DarkModeClasses = ["uk-light", "uk-background-secondary"];
const LightModeClasses = ["uk-dark", "uk-background-default"];

/* FUNCTIONS */

/**
 * Selects the element by the given id and copies its content to the user's clipboard.
 *
 * @note only allowed with SSL connections
 */
export function copyToClipboard(id) {
    var element = document.getElementById(id);

    element.select();
    element.setSelectionRange(0, 99999); /* for mobile */

    navigator.clipboard.writeText(element.value);
}

/**
 * Returns the current UTC date time from the user agent.
 * 
 * @returns the current UTC date time as `Date` instance
 */
export function getCurrentUTC() {
    return new Date().toUTCString();
}

/**
 * Queries an API to get the IP address of the user agent.
 * 
 * @return the IPv4 address of the user agent or an empty string in case of an error
 */
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

/**
 * Returns if the user preferences a dark color scheme.
 * 
 * @returns true if the user uses a dark color scheme or false if not
 */
export function getPrefersColorSchemeDark() {
    return (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
}

/**
 * Sets the site's CSS classes to emulate dark mode.
 */
export function setDarkMode() {
    var htmls = document.getElementsByTagName("html");
    for (let i = 0; i < htmls.length; i++)
        setElementDarkMode(htmls[i]);

    var dropdowns = document.querySelectorAll('.uk-navbar-dropdown, .uk-dropdown-nav, div[uk-dropdown]');
    for (let i = 0; i < dropdowns.length; i++)
        setElementDarkMode(dropdowns[i]);
}

/**
 * Sets the site's CSS classes to emulate light mode.
 */
export function setLightMode() {
    var htmls = document.getElementsByTagName("html");
    for (let i = 0; i < htmls.length; i++)
        setElementLightMode(htmls[i]);

    var dropdowns = document.querySelectorAll('.uk-navbar-dropdown, .uk-dropdown-nav, div[uk-dropdown]');
    for (let i = 0; i < dropdowns.length; i++)
        setElementLightMode(dropdowns[i]);
}

/**
 * Sets the given element's classes to dark mode classes.
 * 
 * @param {DOMElement} element the DOM element to set to dark mode
 */
function setElementDarkMode(element) {
    for (let l = 0; l < LightModeClasses.length; l++)
        element.classList.remove(LightModeClasses[l]);
    for (let d = 0; d < DarkModeClasses.length; d++)
        element.classList.add(DarkModeClasses[d]);
}

/**
 * Sets the given element's classes to light mode classes.
 * 
 * @param {DOMElement} element the DOM element to set to light mode
 */
function setElementLightMode(element) {
    for (let d = 0; d < DarkModeClasses.length; d++)
        element.classList.remove(DarkModeClasses[d]);
    for (let l = 0; l < LightModeClasses.length; l++)
        element.classList.add(LightModeClasses[l]);
}

/**
 * Goes to a given anchor on the local site.
 * 
 * @param {String} anchor the anchor to go to
 */
function goto(anchor) {
    var url = location.href;
    location.href = "#" + anchor;
    history.replaceState(null, null, url);
}

/**
 * Selects a given input element's content and sets the focus
 * 
 * @param {DOMElement} element the input element's id
 */
function selectAndFocus(id) {
    var element = document.getElementById(id);

    element.select();
    element.setSelectionRange(0, 99999); /* for mobile */
    element.focus();
}

/* MOUSETRAP BINDINGS */

Mousetrap.bind('ctrl+1', function (e) {
    goto("numcalculator");
    selectAndFocus("expression");
});

Mousetrap.bind('ctrl+2', function (e) {
    goto("numconverter");
    selectAndFocus("hex");
});

Mousetrap.bind('ctrl+3', function (e) {
    goto("ipcalculator");
    selectAndFocus("ipv4");
});

Mousetrap.bind('ctrl+4', function (e) {
    goto("timestampconverter");
    selectAndFocus("epoch");
});

Mousetrap.bind('ctrl+5', function (e) {
    goto("chmodcalculator");
    selectAndFocus("octal");
});
