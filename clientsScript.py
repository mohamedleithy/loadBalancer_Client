import time
import subprocess

for i in range(500,999):
    
  
    cmd = (
                [
                    "/home/g02-f22/Desktop/loadBalancer_Client/target/debug/loadBalancer_Client", 
                    f'''{i}''',
                    "&"
                
                ]
            )
    start = time.time()
    subprocess.call(cmd)
    end = time.time()
    elapsed = end - start 
    print(f'''Request {i}: sent: {start} received: {end} elapsed: {elapsed}''')

