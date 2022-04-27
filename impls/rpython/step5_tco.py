import sys, traceback
import mal_readline
import mal_types as types
from mal_types import (MalSym, MalInt, MalStr,
                       nil, true, false, _symbol, _keywordu,
                       MalList, _list, MalVector, MalHashMap, MalFunc)
import reader, printer
from env import Env
import core

# read
def READ(str):
    return reader.read_str(str)

# eval
def eval_ast(ast, env):
    if types._symbol_Q(ast):
        assert isinstance(ast, MalSym)
        return env.get(ast)
    elif types._list_Q(ast):
        res = [EVAL(a, env) for a in ast.values]
        return MalList(res)
    elif types._vector_Q(ast):
        res = [EVAL(a, env) for a in ast.values]
        return MalVector(res)
    elif types._hash_map_Q(ast):
        new_dct = {k: EVAL(ast.dct[k], env) for k in ast.dct.keys()}
        return MalHashMap(new_dct)
    else:
        return ast  # primitive value, return unchanged

def EVAL(ast, env):
    while True:
        #print("EVAL %s" % printer._pr_str(ast))
        if not types._list_Q(ast):
            return eval_ast(ast, env)

        # apply list
        if len(ast) == 0: return ast
        a0 = ast[0]
        a0sym = a0.value if isinstance(a0, MalSym) else u"__<*fn*>__"
        if a0sym == u"def!":
            a1, a2 = ast[1], ast[2]
            res = EVAL(a2, env)
            return env.set(a1, res)
        elif a0sym == u"let*":
            a1, a2 = ast[1], ast[2]
            let_env = Env(env)
            for i in range(0, len(a1), 2):
                let_env.set(a1[i], EVAL(a1[i+1], let_env))
            ast = a2
            env = let_env # Continue loop (TCO)
        elif a0sym == u"do":
            if len(ast) == 0:
                return nil
            elif len(ast) > 1:
                eval_ast(ast.slice2(1, len(ast)-1), env)
            ast = ast[-1] # Continue loop (TCO)
        elif a0sym == u"if":
            a1, a2 = ast[1], ast[2]
            cond = EVAL(a1, env)
            if cond is nil or cond is false:
                if len(ast) > 3: ast = ast[3] # Continue loop (TCO)
                else:            return nil
            else:
                ast = a2 # Continue loop (TCO)
        elif a0sym == u"fn*":
            a1, a2 = ast[1], ast[2]
            return MalFunc(None, a2, env, a1, EVAL)
        else:
            el = eval_ast(ast, env)
            f = el.values[0]
            if not isinstance(f, MalFunc):
                raise Exception(f"{f} is not callable")
            if f.ast:
                ast = f.ast
                env = f.gen_env(el.rest()) # Continue loop (TCO) 
            else:
                return f.apply(el.rest())

# print
def PRINT(exp):
    return printer._pr_str(exp)

# repl
def entry_point(argv):
    repl_env = Env()
    def REP(str, env):
        return PRINT(EVAL(READ(str), env))

    # core.py: defined using python
    for k, v in core.ns.items():
        repl_env.set(_symbol(unicode(k)), MalFunc(v))

    # core.mal: defined using the language itself
    REP("(def! not (fn* (a) (if a false true)))", repl_env)

    while True:
        try:
            line = mal_readline.readline("user> ")
            if line == "": continue
            print(REP(line, repl_env))
        except EOFError as e:
            break
        except reader.Blank:
            continue
        except types.MalException as e:
            print(u"Error: %s" % printer._pr_str(e.object, False))
        except Exception as e:
            print("Error: %s" % e)
            #print("".join(traceback.format_exception(*sys.exc_info())))
    return 0

# _____ Define and setup target ___
def target(*args):
    return entry_point

# Just run entry_point if not RPython compilation
import sys
if not sys.argv[0].endswith('rpython'):
    entry_point(sys.argv)
