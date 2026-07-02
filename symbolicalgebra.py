# Symbolic Math Parser
from string import ascii_lowercase as alphabet
from exp import exp
from ln import *
from trig import *
from decimal import Decimal, getcontext
from nthroot import nthroot
getcontext().prec = 35
e = exp(1)

toParse = "sqrt(x)".replace(" ","")
operators = ["+","*","/","-","^","(",")"]
functions = {
    "ln" : [ln, lambda x: Divide(1, x)],
    "log" : [log, lambda x: Divide(1, Mult(
        x,
        ReservedFunction("ln", 2)
    ))], 
    "sin" : [sin, lambda x: ReservedFunction("cos", x)],
    "cos" : [cos, lambda x: Negate(ReservedFunction("sin", x))], 
    "tan" : [tan, lambda x: Pow(
        ReservedFunction("sec", x),
        2
    )], 
    "sec" : [sec, lambda x: Mult(
        ReservedFunction("sec", x),
        ReservedFunction("tan", x)
    )], 
    "csc" : [csc, lambda x: Negate(Mult(
        ReservedFunction("csc", x),
        ReservedFunction("cot", x)
    ))], 
    "cot" : [cot, lambda x: Negate(Pow(
        ReservedFunction("csc", x),
        2
    ))], 
    "sqrt": [lambda x: nthroot(x,n=2),lambda x: Pow(x, Negate(Divide(1,2)))],
    "exp" : '',
    "abs" : [abs,'']
}

constants = {"e" : e, "π": PI}
prettyPrintIndent = "  "
class Node:
    isEvaluable = True
    def __init__(self, term, term2):
        self.termOne = term if not type(term) in (int,float) else Number(Decimal(str(term)))
        self.termTwo = term2 if not type(term2) in (int,float) else Number(Decimal(str(term2)))
        isEvalOne = True
        isEvalTwo = True
        try:
            isEvalOne = term.isEvaluable
        except:
            pass
        try:
            isEvalTwo = term2.isEvaluable
        except:
            pass
        self.isEvaluable = isEvalOne and isEvalTwo
        self.termOne = self.termOne.simplify()
        self.termTwo = self.termTwo.simplify()

    def __str__(self, indent=1):
        ind = prettyPrintIndent * indent
        termOne = self.termOne.__str__(indent+1)
        termTwo = self.termTwo.__str__(indent+1)
        return(f"""{self.__class__.__name__}[
{ind}{termOne},
{ind}{termTwo}
{(indent-1)*prettyPrintIndent}]""")
    def simplify(self):
        return self
    def __eq__(self, o):
        if type(self) != type(o):
            return False
        
        if "termOne" in dir(self):
            if self.termOne != o.termOne:
                return False
        if "termTwo" in dir(self):
            if self.termTwo != o.termTwo:
                return False
        return True

class Undefined:
    isEvaluable = True
    termOne = None
    def evaluate():
        return None

class Negate(Node):
    def __init__(self, termOne):
        self.termOne = termOne
    
    def  __str__(self, indent=1):
        ind = prettyPrintIndent * indent
        return(f"""{self.__class__.__name__}[
{ind}{self.termOne}
{(indent-1)*prettyPrintIndent}]""")

    def  evaluate(self, variables=None):
        if type(self.termOne) == Undefined:
            return Undefined()
        evaled = self.termOne.evaluate(variables).termOne
        if evaled == None:
            return Undefined()
        return Number(-evaled)

    

class Variable(Node):
    isEvaluable = False
    def __init__(self, name):
        self.termOne = name
        self.isEvaluable = False
    def __str__(self, indent=1):
        ind = prettyPrintIndent * indent
        return(f"""{self.__class__.__name__}[
{ind}{self.termOne}
{(indent-1)*prettyPrintIndent}]""")
    def differentiate(self):
        return Number(1)
    def evaluate(self, variables=None):
        return Number(variables[self.termOne])

class Number(Node):
    isEvaluable = True
    def __init__(self, term):
        self.termOne = Decimal(str(term))
    def __str__(self, indent=1):
        return(f"""Number {self.termOne}""")
    def differentiate(self):
        return Number(0)
    def __eq__(self, n):
        if type(n) == Number:
            return self.termOne == n.termOne
        else: 
            return False
    def evaluate(self, variables=None):
        return self

class Add(Node):
    def simplify(self):
        termOne = self.termOne.simplify()
        termTwo = self.termTwo.simplify()
        simplified = Subtract(termOne, termTwo)
        if simplified.termOne == Number(0):
            return simplified.termTwo
        elif simplified.termTwo == Number(0):
            return simplified.termOne
        if type(simplified.termOne) == Negate:
            simple = Subtract(simplified.termTwo, simplified.termOne.termOne)
        elif type(self.termTwo) == Negate:
            simple = Subtract(simplified.termOne, simplified.termTwo.termOne)

        return self

    def differentiate(self):
        derivative = Add(self.termOne.differentiate(), self.termTwo.differentiate()).simplify()
        return derivative

    def evaluate(self, variables=None):
        if self.isEvaluable or not variables == None:
            termOne = self.termOne.evaluate(variables)
            termTwo = self.termTwo.evaluate(variables)
            if type(termOne) == Undefined:
                return Undefined()
            if type(termTwo) == Undefined:
                return Undefined()
            operand1 = termOne.termOne if type(termOne) != Negate else -termOne.termOne.termOne
            operand2 = termTwo.termOne if type(termTwo) != Negate else -termTwo.termOne.termOne
            return Number(operand1 + operand2)

class Subtract(Node):
    def simplify(self):
        termOne = self.termOne.simplify()
        termTwo = self.termTwo.simplify()
        simplified = Subtract(termOne, termTwo)
        if (termOne == Number(0)):
            return Negate(termTwo)
        elif (termTwo == Number(0)):
            return termOne
        return simplified

    def differentiate(self):
        derivative = Subtract(self.termOne.differentiate(), self.termTwo.differentiate())
        if derivative.termTwo == Number(0):
            derivative = derivative.termOne
        return derivative
    def evaluate(self, variables=None):
        if self.isEvaluable or not variables == None:
            termOne = self.termOne.evaluate(variables)
            termTwo = self.termTwo.evaluate(variables)
            if type(termOne) == Undefined:
                return Undefined()
            if type(termTwo) == Undefined:
                return Undefined()
            if termTwo.termOne > termOne.termOne:
                return Negate(Number(termTwo.termOne - termOne.termOne))
            return Number(termOne.termOne - termTwo.termOne)

class Mult(Node):
    def __init__(self, termOne,termTwo):
        super().__init__(termOne, termTwo)
        if type(termTwo) == Number and type(termOne) != Number:
            t = self.termOne
            self.termOne = self.termTwo
            self.termTwo = t

    def simplify(self):
        termOne = self.termOne.simplify()
        termTwo = self.termTwo.simplify()
        simplified = Mult(termOne, termTwo)
        if termOne == Number(0) or termTwo == Number(0):
            return Number(0)
        elif termOne == Number(1):
            return termTwo
        elif termTwo == Number(1):
            return termOne
        elif type(termOne) == Divide and not type(termTwo) == Divide:
            if termOne.termTwo == termTwo and termTwo.isEvaluable:
                return termOne.termOne.simplify()
        elif type(termTwo) == Divide and not type(termOne) == Divide:
            if termTwo.termTwo == termOne and termOne.isEvaluable:
                return termTwo.termOne.simplify()

        if type(simplified.termOne) == Number and type(simplified.termTwo) == Mult:
            if type(simplified.termTwo.termOne) == Number:
                simplified = Mult(
                    Number(
                        simplified.termOne.termOne * simplified.termTwo.termOne.termOne
                    ),
                    simplified.termTwo.termTwo
                )
            elif type(simplified.termTwo.termTwo) == Number:
                simplified = Mult(
                    Number(
                        simplified.termOne.termOne * simplified.termTwo.termTwo.termOne
                    ),
                    simplified.termTwo.termOne
                ) 
        return simplified

    def differentiate(self):
        derivative = Add(
            Mult(
                self.termOne.differentiate(), 
                self.termTwo.simplify(),
            ).simplify(),
            Mult(
                self.termTwo.differentiate(),
                self.termOne.simplify()
            ).simplify()
        ).simplify()
        return derivative
    
    def evaluate(self, variables=None):
        if self.isEvaluable or not variables == None:
            termOne = self.termOne.evaluate(variables)
            termTwo = self.termTwo.evaluate(variables)
            if type(termOne) == Undefined:
                return Undefined()
            if type(termTwo) == Undefined:
                return Undefined()
            return Number(termOne.termOne * termTwo.termOne)

class Divide(Node):
    def evaluate(self, variables=None):
        if self.isEvaluable or not variables == None:
            termOne = self.termOne.evaluate(variables)
            termTwo = self.termTwo.evaluate(variables)
            if termTwo.termOne == 0:
                return Undefined()
            if isinstance(termOne, Negate):
                termOne = termOne.evaluate(variables) 
            if isinstance(termTwo, Negate):
                print("goodbye")
                termTwo = termTwo.evaluate(variables) 
            print(termOne, termTwo)
            return Number(termOne.termOne / termTwo.termOne)
    def differentiate(self):
        if self.isEvaluable:
            return Number(0)
        
        return Divide(
            Subtract(
                Mult(
                    self.termTwo,
                    self.termOne.differentiate(),
                ),
                Mult(
                    self.termOne,
                    self.termTwo.differentiate()
                )
            ),
            Pow(
                self.termTwo,
                2
            )
        )

class Pow(Node):
    def simplify(self):
        termOne = self.termOne.simplify()
        termTwo = self.termTwo.simplify()
        simplified = Pow(termOne, termTwo)

        if simplified.termOne == Number(1):
            return Number(1)
        elif simplified.termOne == Number(0):
            return Number(0)
        elif simplified.termTwo == Number(1):
            return simplified.termOne
        elif simplified.termTwo == Number(0):
            return Number(1)

        return simplified

    def differentiate(self):
        if not self.termOne.isEvaluable:
            if self.termTwo.isEvaluable:
                return Mult(
                    self.termOne.differentiate(),
                    Mult(
                        self.termTwo,
                        Pow(
                            self.termOne,
                            Number(self.termTwo.termOne - 1)
                        )
                    )
                ).simplify()

        return Exponential(
                Mult(
                    ReservedFunction(
                        "ln",
                        self.termOne
                    ),
                    self.termTwo
                )
            ).differentiate()


        return Number(0)
    
    def evaluate(self, variables=None):
        if self.isEvaluable or not variables == None:
            termOne = self.termOne.evaluate(variables)
            termTwo = self.termTwo.evaluate(variables)
            if type(termOne) == Undefined:
                return Undefined()
            if type(termTwo) == Undefined:
                return Undefined()
            if (int(termTwo.termOne) == termTwo.termOne):
                ans = termOne.termOne
                if termTwo.termOne > 0:
                    for n in range(1,int(termTwo.termOne)):
                        ans *= termOne.termOne
                elif termTwo.termOne < 0:
                    ans = 1 / termOne.termOne
                    for n in range(1,abs(int(termTwo.termOne))):
                        ans /= termOne.termOne
                else:
                    return Number(0)
                return Number(ans)
            else: 
                if termOne.termOne < 0:
                    return Undefined()
                return Exponential(
                    Mult(
                        ReservedFunction(
                            "ln",
                            termOne
                        ),
                        termTwo
                    )
                ).evaluate()


class Function(Node):
    def __init__(self, name, term):
        self.name = name
        self.termOne = term
        try:
            self.isEvaluable = term.isEvaluable
        except:
            pass

    def differentiate(self):
        if self.isEvaluable:
            return Number(0)

    def __str__(self, indent=1):
        ind = prettyPrintIndent * indent
        termOne = self.termOne.__str__(indent+1)
        return(f"""{self.__class__.__name__} {self.name}[
{ind}{termOne}
{(indent-1)*prettyPrintIndent}]""")

class Exponential(Function):
    def __init__(self, term):
        self.termOne = term
        try:
            self.isEvaluable = term.isEvaluable
        except:
            pass

    def simplify(self):
        termTwo = self.termOne.simplify()
        simplified = Exponential(termTwo)
        if simplified.termOne == Number(1):
            return Number(1)
        elif simplified.termOne == Number(0):
            return Number(0)
        if type(termTwo) == Mult:
            if type(termTwo.termOne) == ReservedFunction and termTwo.termOne.name == "ln":
                return Pow(termTwo.termOne.termOne, termTwo.termTwo).simplify()
            elif type(termTwo.termTwo) == ReservedFunction and termTwo.termTwo.name == "ln":
                return Pow(termTwo.termTwo.termOne, termTwo.termOne).simplify()

        return simplified

    def differentiate(self):
        if not self.termOne.isEvaluable:
            return Mult(
                self.termOne.differentiate(),
                Exponential(
                    self.termOne
                )
            ).simplify()
    
    def evaluate(self, variables=None):
        if self.isEvaluable or not variables == None:
            termOne = self.termOne.evaluate(variables)
            if type(termOne) == Undefined:
                return Undefined()
            return Number(exp(termOne.termOne))
    
    def __str__(self, indent=1):
        ind = prettyPrintIndent * indent
        termOne = self.termOne.__str__(indent+1)
        return(f"""{self.__class__.__name__} [
{ind}{termOne}
{(indent-1)*prettyPrintIndent}]""")

class UserFunction(Function):
    pass

class ReservedFunction(Function):
    def evaluate(self, variables=None):
        if self.name == "ln" and self.termOne.evaluate(variables).termOne < 0:
            raise ValueError("Log undefined for x < 0")
        ans = functions[self.name][0](self.termOne.evaluate(variables).termOne)
        if ans != None:
            return Number(ans)
        else:
            return Undefined()
    def differentiate(self):
        if self.isEvaluable:
            return Number(0)
        
        return Mult(
            self.termOne.differentiate(),
            functions[self.name][1](self.termOne)
        ).simplify()

class Constant(Node):
    isEvaluable = True
    def __init__(self, name):
        self.termOne = name
    def __str__(self, indent=1):
        return(f"""Constant {self.termOne}""")
    def differentiate(self):
        return Number(0)
    def __eq__(self, n):
        if type(n) == Constant :
            return self.termOne == n.termOne
        else: 
            return False

    def evaluate(self, variables=None):
        return Number(constants[self.termOne])

ops = [Add, Subtract, Mult, Divide, Pow, Variable]
def safeIndex(lis, char):
    if char in lis:
        return lis.index(char)
    else:
        return len(lis)

def tree(parsed1):
    parsed = list(parsed1)
    order = ['^','*/',"+-"]
    ind = 0
    for item in parsed:
        if type(item) == list:
            parsed[parsed.index(item)] = tree(item)
        elif item in list(functions.keys()):
            ind = parsed.index(item)
            parsed[ind] = ReservedFunction(item, tree(parsed[ind+1]))
            del parsed[ind+1]
        elif type(item) == Decimal or item.isnumeric():
            parsed[ind] = Number(Decimal(str(item)))
        ind += 1

    for op in ["+","-"]:
        ind = 0
        while ind < len(parsed):
            if (parsed[ind] in list(constants.keys())):
                parsed[ind] = Constant(parsed[ind])
            elif (str(parsed[ind]) in alphabet):
                parsed[ind] = Variable(parsed[ind])
            if not (ind < len(parsed)):
                break
            if (ind-1 == -1 or parsed[ind-1] in operators[:-1]) and parsed[ind] == op:
                
                opc = parsed[ind]
                parsed.remove(opc)
                
                if op == "-":
                    try:
                        parsed[ind] = -parsed[ind]
                    except:
                        parsed[ind] = Negate(parsed[ind])
                ind-=1
            ind+=1

    for op in order:
        if len(op) == 2:
            while op[0] in parsed or op[1] in parsed:
                ind = parsed.index(op[0]) if safeIndex(parsed,op[0]) < safeIndex(parsed,op[1]) else parsed.index(op[1])
                opc = parsed[ind]
                parsed.remove(opc)

                op1 = parsed[ind-1]
                op2 = parsed[ind]
                match opc:
                    case "*":
                        parsed.insert(ind, Mult(op1,op2))
                    case "/":
                        parsed.insert(ind, Divide(op1,op2))
                    case "+":
                        parsed.insert(ind, Add(op1,op2))
                    case "-":
                        parsed.insert(ind, Subtract(op1,op2))
                del parsed[ind-1]
                del parsed[ind]
        else: 
            while op in parsed:
                ind = parsed.index(op)
                parsed.remove(op)
                op1 = parsed[ind-1]
                op2 = parsed[ind]
                del parsed[ind]
                del parsed[ind-1]
                if op1 == Constant("e"):
                    parsed.insert(ind-1, Exponential(
                        op2
                    ))
                else:
                    parsed.insert(ind-1, Pow(op1,op2))
                
    return parsed[0]


class Pair:
    def __init__(self,arg1,arg2):
        self.pair = (arg1, arg2)
        self.children = []
    
    def addChild(self, pair):
        s = pair.pair[1] - self.pair[0]-1
        newPair = Pair(pair.pair[0] - self.pair[0]-1, s)
        newPair.parent = self
        newPair.children = pair.children
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
        char = list(self.string)
        reconstructed = [""]
        i = 0
        while i < len(char):
            foundFunction = False
            for function in list(functions.keys()):
                if self.string[i:i+len(function)] == function:
                    foundFunction = True
                    reconstructed.append("")
                    reconstructed[-1] = function
                    reconstructed.append("")
                    i = i+len(function)
                    break
            if foundFunction:
                continue   
            if char[i] not in operators:
                reconstructed[-1] += char[i]
            else:
                if char[i] == "(" and ((len(reconstructed) == 1 and reconstructed[0] not in operators+list(functions.keys())) or reconstructed[-2] not in operators+list(functions.keys())):
                    reconstructed.append("")
                    if reconstructed[0] != "":
                        reconstructed[-1] = "*"
                    reconstructed.append("")

                reconstructed.append("")
                reconstructed[-1] = char[i]
                reconstructed.append("")
            i+=1
                


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
        for term in range(0, len(reconstructed)):
            try:
                reconstructed[term] = Decimal(reconstructed[term])
            except:
                pass
        term = 0
        while (term < len(completedPairs)-1):
            if completedPairs[term+1].contains(completedPairs[term]):
                completedPairs[term+1].addChild(completedPairs[term])
                del completedPairs[term]
                term = -1
            term += 1
            if not (term < len(completedPairs)-1):
                break
        completedPairs.reverse()
        for pair in completedPairs:
            reconstructed = self.recursiveParen(reconstructed, pair)

        return reconstructed

