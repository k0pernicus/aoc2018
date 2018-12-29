import os
import sequtils
import strutils
import system
import tables

type lines = seq[string]

proc parse(filePath: string): lines =
  result = readFile(filePath).strip().splitLines()

proc compare(s1, s2: string): int =
  result = s1.zip(s2).filterIt(it.a != it.b).len

proc getBase(s1, s2: string): string =
  result = s1.zip(s2).filterIt(it.a == it.b).mapIt(it.a).join

proc count(line: string): (int, int) =
  let characters = toSeq(line.items)
  if len(characters) == 0:
    return (0, 0)
  var
    two_n = 0
    three_n = 0
  for n in newCountTable(characters).values():
    if n == 2 and two_n != 1:
      two_n = 1
    if n == 3 and three_n != 1:
      three_n = 1
  result = (two_n, three_n)

proc solvePart1(l: lines) = 
  var
    two_n = 0
    three_n = 0
  for line in l:
    let (x, y) = count(line)
    two_n += x
    three_n += y
  echo two_n, " * ", three_n, " = ", two_n * three_n

proc solvePart2(l: lines) =
  for i in 0..<len(l):
    for j in 0..<len(l):
      if i == j:
        continue
      if compare(l[i], l[j]) == 1:
        echo "> Found ", getBase(l[i], l[j])

proc main() =
  if paramCount() == 0:
    echo "Please to add the file as program argument"
    quit(QuitFailure)
  let filePath = paramStr(1)
  if not existsFile(filePath):
    echo "Unfortunately, this file does not exists in your system"
    quit(QuitFailure)
  let l = parse(filePath)
  solvePart1(l)
  solvePart2(l)

when isMainModule:
  main()
