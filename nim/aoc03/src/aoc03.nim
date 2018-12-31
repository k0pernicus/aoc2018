import os
import sequtils
import sets
import system
import strformat
import strscans
import strutils

const
  H = 1000
  W = 1000

type ID = int

# Fabric is a matrix of array of W * H IDs array type
type Fabric = seq[seq[seq[ID]]]

type Rectangle = object
  id, x, y, h, w: int

type Rectangles = seq[Rectangle]

proc parse(filePath: string): Rectangles =
  let lines = readFile(filePath).strip().splitLines()
  result = newSeq[Rectangle]()
  for line in lines:
    var id, x, y, h, w : int
    if line.strip().scanf("#$i @ $i,$i: $ix$i", id, x, y, w, h):
      result.add(Rectangle(id: id, x: x, y: y, h: h, w: w))

proc toFabric(rectangles: Rectangles): Fabric =
  echo "Instanciating the fabric..."
  result = newSeqWith(W, newSeqWith(H, newSeq[ID]()))
  for rectangle in rectangles:
    for i in rectangle.x .. < (rectangle.x + rectangle.w):
      for j in rectangle.y .. < (rectangle.y + rectangle.h):
        result[i][j].add(rectangle.id)

proc solvePart1(fabric: Fabric): int =
  var squares = 0
  for i in 0..<W:
    for j in 0..<H:
      if len(fabric[i][j]) > 1:
        squares += 1
  result = squares

proc solvePart2(fabric: Fabric, rectangles: Rectangles): int =
  var IDs: HashSet[ID] = toSet(rectangles.map(proc(x: Rectangle): ID = x.id))
  for i in 0..<W:
    for j in 0..<H:
      let cSquare = fabric[i][j]
      if len(cSquare) > 1:
        for id in cSquare:
          IDs.excl(id) 
  assert(len(IDs) == 1)
  result = IDs.pop()

proc main() =
  if paramCount() == 0:
    echo "Please to add the file as program argument"
    quit(QuitFailure)
  let filePath = paramStr(1)
  if not existsFile(filePath):
    echo "Unfortunately, this file does not exists in your system"
    quit(QuitFailure)
  let rectangles = parse(filePath)
  let fabric = toFabric(rectangles)
  echo "The number of square inches (with 2+ claims) is ", solvePart1(fabric)
  echo "The unique ID (that does not overlap) is ", solvePart2(fabric, rectangles)

when isMainModule:
  main()
