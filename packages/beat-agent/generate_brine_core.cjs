const { drumClip, melodyClip, writeClipsToMidi } = require('./src/index');

const kick  = drumClip({ pattern: 'x---x---x---x---', instrument: 'C1' });
const snare = drumClip({ pattern: '----x-------x---', instrument: 'D1' });
const hat   = drumClip({ pattern: 'x-x-x-x-x-x-x-x-', instrument: 'F#1' });
const bass  = melodyClip({ REDACTED_SECRET: 'C1', scaleName: 'minor', degrees: [0], pattern: 'x---x---x---x---' });
const glitch = melodyClip({ REDACTED_SECRET: 'C2', scaleName: 'minor', degrees: [0, 1], pattern: '---x--x--x---x---' });

writeClipsToMidi([kick, snare, hat, bass, glitch], '/home/err/devel/Music/brine_core_loop.mid');
console.log('MIDI saved to Music/brine_core_loop.mid');
