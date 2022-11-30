accumulator = 0
numberOfReq = 1000000

# time elapsed for request 999 client 500: 23.333µs 
# time elapsed for request 988 client 500: 26.494416ms
def readFile(filename): 
    log = open(filename, 'r')
    lines = log.readlines()
    global accumulator
    for line in lines[:-1]: 
        time = getTime(line)
        accumulator = accumulator + time

    


def getTime(line):
    
    line = line.split(':')
    line = line[1]
    unit = line[len(line) - 2]
    line = line.strip()
    line = line[:-2]

    if(unit == 'µ'):
        print(line)
        return float(line)/1000
    print(line)
    return float(line)


def getAverage(): 

    average = accumulator/numberOfReq
    print(f'''average request time: {average}''')

def main(): 

    readFile('/Users/leithy/Desktop/loadBalancer/loadBalancer_Client/logs/test1_clientA/client500.txt')   
    #getTime("time elapsed for request 999 client 500: 23.333µs")
    getAverage()

main()    
