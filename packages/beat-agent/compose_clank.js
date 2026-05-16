import { drumClip, melodyClip, writeClipsToMidi } from './src/index.js';

const kick  = drumClip({ pattern: 'x---x---', instrument: 36 });
const snare = drumClip({ pattern: '--x---x-', instrument: 38 });
const hat   = drumClip({ pattern: 'x-x-x-x-', instrument: 42 });
const bass  = melodyClip({ REDACTED_SECRET: 'C2', scaleName: 'minor', pattern: 'x-x-x-x-' });
const lead  = melodyClip({ REDACTED_SECRET: 'C3', scaleName: 'minor', pattern: 'x-x-x-x-x-x-x-x' });

await writeClipsToMidi([kick, snare, hat, bass, lead], '/tmp/clanker_beat.mid');
console.log('Beat composed at /tmp/clanker_beat.mid');

