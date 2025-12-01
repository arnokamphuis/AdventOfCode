import argparse
import subprocess
import webbrowser as wb
import os
import shutil

session_file = open("session.txt", "r")
SESSION = session_file.readlines()[0].strip()

useragent = 'https://github.com/arnokamphuis/AdventOfCode/blob/master/prepare.py'
parser = argparse.ArgumentParser(description='Read input')
parser.add_argument('--year', type=int, default=24)
parser.add_argument('--day', type=int, default=1)
args = parser.parse_args()

year = 2000 + args.year
day  = args.day

os.mkdir('day{:02d}-{}'.format(day,args.year))

cmd = f'curl https://adventofcode.com/20{args.year}/day/{args.day}/input --cookie "session={SESSION}" -A {useragent}'
output = subprocess.check_output(cmd, shell=True)
output = output.decode('utf-8')
real_output = '\n'.join(output.split('\n'))

print('\n'.join(output.split('\n')[:10]))

real_input_file = open("day{:02d}-{}/day{:02d}-real.txt".format(day,args.year,day), "w+")
real_input_file.write(real_output)
real_input_file.close()

test_input_file = open('day{:02d}-{}/day{:02d}-test.txt'.format(day,args.year,day), "w+")
test_input_file.write("")
test_input_file.close()

shutil.copy('template.py', 'day{:02d}-{}/day{:02d}-{}.py'.format(day,args.year,day,args.year))
subprocess.Popen('git add day{:02d}-{}/*'.format(day, args.year), shell=True)

wb.open(f'https://adventofcode.com/{year}/day/{day}')
