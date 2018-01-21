const { exec } = require('child_process');
const fs = require('fs');
const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

~function loop() {
    rl.question('> ', (answer) => {
        if (answer == '#') {
            exec('./build.sh', (err, stdout, stderr) => {
                console.log(stdout, stderr);
                loop();
            });
        }
        else {
            fs.writeFileSync('_repl.asm', '\t' + answer, 'utf8');
            exec('env AS_MSGPATH=msg ./asl -xx -c -q -A _repl.asm', (err, stdout, stderr) => {
                // console.error(stderr);
                exec('./s3p2bin _repl.p _repl.bin _repl.h', (err, stdout, stderr) => {
                    // console.error(stderr);
                    let output = fs.readFileSync('_repl.bin');
                    console.log(Array.from(output).map(d => d.toString(16)));
                    console.log(Array.from(output).map(d => d.toString(2)));
                    fs.unlinkSync('_repl.asm');
                    fs.unlinkSync('_repl.p');
                    fs.unlinkSync('_repl.h');
                    fs.unlinkSync('_repl.bin');
                    loop();
                });

            });
        }
    });
} ();
