import sys, copy, types as pytypes

# python 3.0 differences
if sys.hexversion > 0x3000000:
    _u = lambda x: x
    _s2u = lambda x: x
else:
    import codecs
    _u = lambda x: codecs.unicode_escape_decode(x)[0]
    _s2u = lambda x: unicode(x)

str_types = [str] if sys.version_info[0] >= 3 else [str, unicode]

# General functions

def _equal_Q(a, b):
    ota, otb = type(a), type(b)
    if _string_Q(a) and _string_Q(b):
        return a == b
    if not (ota == otb or (_sequential_Q(a) and _sequential_Q(b))):
        return False;
    if (
        _symbol_Q(a)
        or not _symbol_Q(a)
        and not _list_Q(a)
        and not _vector_Q(a)
        and not _hash_map_Q(a)
    ):
        return a == b
    elif not _symbol_Q(a) and (_list_Q(a) or _vector_Q(a)):
        if len(a) != len(b): return False
        return all(_equal_Q(a[i], b[i]) for i in range(len(a)))
    else:
        akeys = sorted(a.keys())
        bkeys = sorted(b.keys())
        if len(akeys) != len(bkeys): return False
        for i in range(len(akeys)):
            if akeys[i] != bkeys[i]: return False
            if not _equal_Q(a[akeys[i]], b[bkeys[i]]): return False
        return True

def _sequential_Q(seq): return _list_Q(seq) or _vector_Q(seq)

def _clone(obj):
    if type(obj) != pytypes.FunctionType:
        return copy.copy(obj)
    if obj.__code__:
        return pytypes.FunctionType(
                obj.__code__, obj.__globals__, name = obj.__name__,
                argdefs = obj.__defaults__, closure = obj.__closure__)
    else:
        return pytypes.FunctionType(
                obj.func_code, obj.func_globals, name = obj.func_name,
                argdefs = obj.func_defaults, closure = obj.func_closure)

#
# Exception type
#

class MalException(Exception):
    def __init__(self, object):
        self.object = object

# Scalars
def _nil_Q(exp):    return exp is None
def _true_Q(exp):   return exp is True
def _false_Q(exp):  return exp is False
def _string_Q(exp):
    return (
        len(exp) == 0 or exp[0] != _u("\u029e")
        if type(exp) in str_types
        else False
    )
def _number_Q(exp): return type(exp) == int

# Symbols
class Symbol(str): pass
def _symbol(str): return Symbol(str)
def _symbol_Q(exp): return type(exp) == Symbol

# Keywords
# A specially prefixed string
def _keyword(str):
    return str if str[0] == _u("\u029e") else _u("\u029e") + str
def _keyword_Q(exp):
    return (
        len(exp) != 0 and exp[0] == _u("\u029e")
        if type(exp) in str_types
        else False
    )

# Functions
def _function(Eval, Env, ast, env, params):
    def fn(*args):
        return Eval(ast, Env(env, params, List(args)))
    fn.__meta__ = None
    fn.__ast__ = ast
    fn.__gen_env__ = lambda args: Env(env, params, args)
    return fn
def _function_Q(f):
    return callable(f)

# lists
class List(list):
    def __add__(self, rhs): return List(list.__add__(self, rhs))
    def __getitem__(self, i):
        if type(i) == slice: return List(list.__getitem__(self, i))
        elif i >= len(self): return None
        else:                return list.__getitem__(self, i)
    def __getslice__(self, *a): return List(list.__getslice__(self, *a))
def _list(*vals): return List(vals)
def _list_Q(exp):   return type(exp) == List


# vectors
class Vector(list):
    def __add__(self, rhs): return Vector(list.__add__(self, rhs))
    def __getitem__(self, i):
        if type(i) == slice: return Vector(list.__getitem__(self, i))
        elif i >= len(self): return None
        else:                return list.__getitem__(self, i)
    def __getslice__(self, *a): return Vector(list.__getslice__(self, *a))
def _vector(*vals): return Vector(vals)
def _vector_Q(exp): return type(exp) == Vector

# Hash maps
class Hash_Map(dict): pass
def _hash_map(*key_vals):
    hm = Hash_Map()
    for i in range(0,len(key_vals),2): hm[key_vals[i]] = key_vals[i+1]
    return hm
def _hash_map_Q(exp): return type(exp) == Hash_Map

# atoms
class Atom(object):
    def __init__(self, val):
        self.val = val
def _atom(val): return Atom(val)
def _atom_Q(exp):   return type(exp) == Atom

def py_to_mal(obj):
        if type(obj) == list:   return List(obj)
        if type(obj) == tuple:  return List(obj)
        elif type(obj) == dict: return Hash_Map(obj)
        else:                   return obj
