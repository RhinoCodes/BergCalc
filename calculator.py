# Symbolic Math Parser

toParse = "5(30)".replace(" ","")
operators = ["+","*","/","-","^","(",")"]

def safeIndex(lis, char):
    if char in lis:
        return lis.index(char)
    else:
        return len(lis)

def calculate(parsed):
    order = ['^','*/',"+-"]
    for item in parsed:
        if type(item) == list:
            parsed[parsed.index(item)] = str(calculate(item))
    for op in order:
        if len(op) == 2:
            while op[0] in parsed or op[1] in parsed:
                ind = parsed.index(op[0]) if safeIndex(parsed,op[0]) < safeIndex(parsed,op[1]) else parsed.index(op[1])
                opc = parsed[ind]
                parsed.remove(opc)

                op1 = float(parsed[ind-1])
                op2 = float(parsed[ind])
                match opc:
                    case "*":
                        parsed.insert(ind, str(op1*op2))
                    case "/":
                        parsed.insert(ind, str(op1/op2))
                    case "+":
                        parsed.insert(ind, str(op1+op2))
                    case "-":
                        parsed.insert(ind, str(op1-op2))
                del parsed[ind-1]
                del parsed[ind]
        else: 
            while op in parsed:
                ind = parsed.index(op)
                parsed.remove(op)
                parsed.insert(ind, str(float(parsed[ind-1])**float(parsed[ind])))
                del parsed[ind-1]
                del parsed[ind]

    return float(parsed[0])


class Pair:
    def __init__(self,arg1,arg2):
        self.pair = (arg1, arg2)
        self.children = []
    
    def addChild(self, pair):
        s = pair.pair[1] - self.pair[0]-1
        newPair = Pair(pair.pair[0] - self.pair[0]-1, s)
        newPair.parent = self
        self.children.append(newPair)

    def contains(self, pair):
        return self.pair[0] < pair.pair[0] and self.pair[1] > pair.pair[1]

    def __str__(self):
        return f"[{self.pair[0]}, {self.pair[1]}]"

    def __len__(self):
        return self.pair[1] - self.pair[0]

class Parser:
    def __init__(self, parse):
        self.string = parse

    def recursiveParen(self, parse, pair):
        pairList = parse[pair.pair[0]+1:pair.pair[1]]
        if (pair.pair[1] != -1):
            del parse[pair.pair[0]:pair.pair[1]+1]
        else:
            del parse[pair.pair[0]:pair.pair[1]]
            del parse[len(parse)-1]
        for child in pair.children:
            pairList = self.recursiveParen(pairList, child)
        parse.insert(pair.pair[0], pairList)
        return parse



    def firstPass(self):
        char = list(toParse)
        reconstructed = [""]
        for i in range(0, len(char)):
            if char[i] not in operators:
                reconstructed[-1] += char[i]
            else:
                if char[i] == "(" and char[i-1] not in operators:
                    reconstructed.append("")
                    reconstructed[-1] = "*"
                    reconstructed.append("")

                reconstructed.append("")
                reconstructed[-1] = char[i]
                reconstructed.append("")
                


        term=0
        while term < len(reconstructed):
            if reconstructed[term].__contains__("x") and reconstructed[term].index("x") > 0:
                reconstructed[term] = reconstructed[term][0:reconstructed[term].index("x")]
                reconstructed.insert(term+1, "*")
                reconstructed.insert(term+2,"x") 
            term+=1

        while '' in reconstructed:
            reconstructed.remove('')
        
        completedPairs = []
        pairs = []
        term = 0
        while term < len(reconstructed):
            if reconstructed[term] == "(":
                pairs.append(term)
            elif reconstructed[term] == ")":
                completedPairs.append(Pair(pairs[-1], term))
                del pairs[-1]
            term+=1
        if len(pairs) != 0:
            raise ValueError("Unclosed Parenthesis")

        term = 0
        pairs = []
        while term < len(completedPairs)-1:
            if completedPairs[term+1].contains(completedPairs[term]):
                completedPairs[term+1].addChild(completedPairs[term])
                del completedPairs[term]
            term += 1
        completedPairs.reverse()
        for pair in completedPairs:
            reconstructed = self.recursiveParen(reconstructed, pair)

        return reconstructed