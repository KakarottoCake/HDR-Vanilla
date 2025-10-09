var enabled = [];
var str = "";
function toggle(mode_name, self, elementID) {
  const text = document.getElementById(elementID).innerHTML;
  if (enabled.includes(mode_name)) {
    enabled.splice(enabled.indexOf(mode_name), 1);
  } else {
    enabled.push(mode_name);
  }
  var x = document.getElementById(elementID);
  if (x.style.display === "none") {
    x.style.display = "block";
  } else {
    x.style.display = "none";
  }
  self.blur();
  self.focus();
  str = enabled.join("-");
}

function textFocus(text) {
  document.getElementById("focus-text").innerHTML = text;
}

var currentModeIndex = 0;
var gameModes = [
  { id: '', name: 'Select a Game Mode', desc: 'Click to toggle various alternate system mechanics!' },
  { id: 'smash64', name: 'Smash 64 Mode', desc: 'Clash in classic fashion! Removes DI, airdodges, walltechs, and landing lag. Also raises hitstun and shieldstun, and alters character physics.' },
  { id: 'rivalsofaether', name: 'Rivals of Aether Mode', desc: 'Removes shields, grabs, ledges, & spotdodges, and walljump from specials, hitfalling, and improved movement!' }
];

function cycleGameMode(elementID) {
  // Cycle to next mode
  currentModeIndex = (currentModeIndex + 1) % gameModes.length;
  const currentMode = gameModes[currentModeIndex];

  // Update button text
  const button = document.getElementById('gamemode-cycle-button');
  if (button) {
    button.querySelector('h2').textContent = currentMode.name;
  }

  // Update focus text
  textFocus(currentMode.desc);

  // Remove all game modes from enabled array
  gameModes.forEach(mode => {
    if (mode.id && enabled.includes(mode.id)) {
      enabled.splice(enabled.indexOf(mode.id), 1);
    }
    // Hide checkmark
    if (mode.id) {
      const checkmark = document.getElementById(mode.id);
      if (checkmark) {
        checkmark.style.display = "none";
      }
    }
  });

  // If a valid mode was selected, enable it
  var x = document.getElementById(elementID);
  if (currentMode.id && currentMode.id !== "") {
    enabled.push(currentMode.id);
    // Show checkmark
    x.style.display = "block";
  } else {
    x.style.display = "none";
  }

  str = enabled.join("-");
}

function saveAndExit() {
  if (enabled === undefined || enabled.length == 0) {
    location.href = "http://localhost/";
  }
  str = enabled.join("-");
  location.href = "http://localhost/" + str;
}
