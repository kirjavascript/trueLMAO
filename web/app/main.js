import init from '../pkg/web.js';
import gamepad from './gamepad.js';

const canvas = document.querySelector('canvas');
const ctx = canvas.getContext('2d', { alpha: false });
const img = ctx.createImageData(320, 240);

(async () => {
    const emu = await init('emu.wasm');

    function draw() {
        const buffer = new Uint8ClampedArray(
            emu.memory.buffer, emu.screen(), 320 * 240 * 3
        );
        for (let i = 0; i < 320*240; i++) {
            const bufferIndex = i * 3;
            const imgIndex = i * 4;
            img.data[imgIndex+0] = buffer[bufferIndex+0];
            img.data[imgIndex+1] = buffer[bufferIndex+1];
            img.data[imgIndex+2] = buffer[bufferIndex+2];
            img.data[imgIndex+3] = 255;
        }

        ctx.putImageData(img, 0, 0);

        // ctx.putImageData(new ImageData(buffer, 320), 0, 0);

        // TODO: webgl
    }

    const frameCount = document.querySelector('.frameCount');
    const epoch = performance.now();
    let framesDone = 0;
    const loop = () => {
        requestAnimationFrame(loop);

        const diff = performance.now() - epoch;
        const frames = diff * 0.06 | 0; // real value is 0.05992274
        const frameAmount = frames - framesDone;
        frameCount.textContent = String(frameAmount);

        if (document.visibilityState !== 'hidden') {

            emu.gamepad_p1(gamepad());

            if (frameAmount > 5) {
                emu.frame(true);
            } else {
                for (let i = 0; i < frameAmount; i++) {
                    emu.frame(i === frameAmount - 1);
                }
            }
            if (frameAmount > 0) {
                draw();
            }
        }
        framesDone = frames;
    };
    loop();
})();
