// create web audio api context
const Audio = window.AudioContext || window.webkitAudioContext;

// create Oscillator node
let audio = new Audio();
let oscillator = audio.createOscillator();

oscillator.type = "triangle";
// oscillator.frequency.setValueAtTime(440, 0);
// oscillator.frequency.setValueAtTime(540, 1);
// oscillator.frequency.setValueAtTime(640, 2);
// oscillator.frequency.setValueAtTime(740, 3);
// oscillator.frequency.setValueAtTime(840, 4);

oscillator.frequency.setValueAtTime(1250, 0);
oscillator.frequency.setValueAtTime(500, 0.7);
// oscillator.frequency.setValueAtTime(600, 0.9);
// oscillator.frequency.setValueAtTime(700, 1.4);
oscillator.connect(audio.destination);
oscillator.start();

setTimeout(() => oscillator.stop(), 3000);
