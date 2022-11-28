import time
import subprocess

for i in range(0,10):
    cmd = (
            [
                "cargo",
                "run",
                "--", 
                f'''{i}'''
            
            ]
        )
    start = time.time()
    subprocess.check_call(cmd)
    end = time.time()
    elapsed = end - start 
    print(f'''Request {i}: sent: {start} received: {end} elapsed: {elapsed}''')

