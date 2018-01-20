const { exec } = require('child_process');
const fs = require('fs');
const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

~function loop() {
    rl.question('> ', (answer) => {
        if (answer == '_') {
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
                    let output = fs.readFileSync('_repl.bin', 'utf8');
                    console.log([...output].map(d => {
                        return d.charCodeAt(0).toString(16).padStart(2, '0');
                    }).join` `);
                    console.log([...output].map(d => {
                        return d.charCodeAt(0).toString(2).padStart(8, '0');
                    }).join` `);
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
