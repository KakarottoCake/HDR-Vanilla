var enabled_modes = new Map();
enabled_modes.set('tag', false);
enabled_modes.set('turbo', false);
enabled_modes.set('hitfall', false);
enabled_modes.set('airdash', false);
enabled_modes.set('smash64', false);
enabled_modes.set('magicseries', false);
enabled_modes.set('element', false);
enabled_modes.set('rivalsofaether', false);

function toggle_mode(mode_name, self) {
    if (event && event.keyCode !== 13) return
    //alert("toggling: " + mode_name);
    var mode_key = mode_name.replace(/\s+/g, '').toLowerCase();
    var is_now_enabled = !enabled_modes.get(mode_key);
    enabled_modes.set(mode_key, is_now_enabled);
    
    // If toggling Rivals of Aether, also toggle Hitfall
    if (mode_key === 'rivalsofaether') {
        enabled_modes.set('hitfall', is_now_enabled);
        var hitfall_element = document.getElementById('hitfall-text-field');
        if (hitfall_element) {
            hitfall_element.innerHTML = "Hitfall Mode (" + (is_now_enabled ? "ON" : "OFF") + ")";
        }
    }
    
    var button_text = mode_name + " Mode (" + (is_now_enabled ? "ON" : "OFF") + ")";
    var element = document.getElementById(mode_key + "-text-field");
    element.innerHTML = button_text;
    self.blur();
    self.focus();
}

function exit_with_id() {
    if (event && event.keyCode !== 13) return
    var str = "";
    enabled_modes.forEach((v, k) => {
        if (v === true) {
            str += k + '-';
        }
    });
    if (str.endsWith('-')) {
        str = str.substring(0, str.length - 1);
    }
    if (str.length == 0) {
        str = "none";
    }
    //alert("exiting with str: " + str);
    location.href = "http://localhost/" + str;
}