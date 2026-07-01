from decimal import Decimal, getcontext, ROUND_HALF_UP
from symbolicalgebra  import Parser, tree
from nthroot  import nthroot

SQRT2 = nthroot(2)
getcontext().prec = 50

toParse = "x^(-0.5)"

nodes_k = [
    Decimal(0),
    Decimal("2.077849550078984676006894037732449e-01"),
    Decimal("4.058451513773971669066064120769615e-01"),
    Decimal("5.860872354676911302941448382587296e-01"),
    Decimal("7.415311855993944398638647732807884e-01"),
    Decimal("8.648644233597690727897127886409262e-01"),
    Decimal("9.491079123427585245261896840478513e-01"),
    Decimal("9.914553711208126392068546975263285e-01"),
]

weights_k = [
    Decimal("2.094821410847278280129991748917143e-01"),
    Decimal("2.044329400752988924141619992346491e-01"),
    Decimal("1.903505780647854099132564024210137e-01"),
    Decimal("1.690047266392679028265834265985503e-01"),
    Decimal("1.406532597155259187451895905102379e-01"),
    Decimal("1.047900103222501838398763225415180e-01"),
    Decimal("6.309209262997855329070066318920429e-02"),
    Decimal("2.293532201052922496373200805896959e-02")
]

gauss_nodes = [
    nodes_k[0], nodes_k[2], nodes_k[4],nodes_k[6]
]

gauss_weights = [
    Decimal("4.179591836734693877551020408163265e-01"),
    Decimal("3.818300505051189449503697754889751e-01"),
    Decimal("2.797053914892766679014677714237796e-01"),
    Decimal("1.294849661688696932706114326790820e-01")
]

def integrate(func, a, b, nodes=nodes_k, weights=weights_k, tol = Decimal("1e-5")):
    ans = 0
    a = Decimal(str(a))
    b = Decimal(str(b))
    if b > a:
        return - integrate(func, b, a, nodes, weights, tol)
    for i in range(len(nodes)):
        x = ((nodes[i] + 1) * (b - a) / 2) + a
        w = weights[i] * ((b - a) / 2)
        if nodes[i] !=0:
            ans += func(((-nodes[i] + 1) * (b - a) / 2) + a) * w

        ans += func(x) * w
    if nodes != gauss_nodes:
        error = abs(ans - integrate(func, a, b, gauss_nodes, gauss_weights))
        if error > tol:
            ans = integrate(func, a, (b+a)/2, nodes_k, weights_k, tol) + integrate(func, (b+a)/2, b, nodes_k, weights_k,tol)
    return Decimal(f"{ans:.10g}")