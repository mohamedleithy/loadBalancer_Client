import os
directory = 'logs/test1_clientA'



accumulator = 0
numberOfReq = 0
milli_counter = 0
micro_counter = 0

# time elapsed for request 999 client 500: 23.333µs 
# time elapsed for request 988 client 500: 26.494416ms
def readFile(filename): 
    log = open(filename, 'r')
    lines = log.readlines()
    global accumulator
    global numberOfReq
    for line in lines[:-1]: 
        numberOfReq = numberOfReq+1
        time = getTime(line)
        accumulator = accumulator + time

    


def getTime(line):
    global milli_counter
    global micro_counter
    
    line = line.split(':')
    line = line[1]
    line = line.strip()
    unit = line[len(line) - 2]
    line = line[:-2]

    
    if(unit == 'µ'):
        micro_counter = micro_counter + 1
        return float(line)/1000
    
    milli_counter = milli_counter + 1
    return float(line)


def getAverage(): 

    average = accumulator/numberOfReq
    print(f'''average request time: {average}''')
    
def loopInFolder(folder_name):
    for filename in os.listdir(directory):
        if filename != '.DS_Store':
            readFile(folder_name +filename)
    

def main(): 

    loopInFolder('logs/test6_clientB/')

    #getTime("time elapsed for request 999 client 500: 23.333µs")
    getAverage()

main()    
