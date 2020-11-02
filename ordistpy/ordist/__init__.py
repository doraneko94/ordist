import numpy as np
from ordist import *

def parser(v1, v2, mode):
    global a, id1, id2, elem_to_id, id_to_elem
    global id_flg
    id_flg = False
    if type(v1) == np.ndarray:
        v1 = v1.tolist()
    if type(v2) == np.ndarray:
        v2 = v2.tolist()
    if type(v1[0]) == int:
        a = OrDistInt(v1, v2)
    elif type(v1[0]) == str:
        a = OrDistStr(v1, v2)
    else:
        elem_to_id = dict()
        id_to_elem = dict()
        id1 = [0 for _ in range(len(v1))]
        id2 = [0 for _ in range(len(v2))]
        count = 0
        for i, e in enumerate(v1):
            if e in elem_to_id:
                id1[i] = elem_to_id[e]
            else:
                elem_to_id[e] = count
                id_to_elem[count] = e
                id1[i] = count
                count += 1
        for i, e in enumerate(v2):
            if e in elem_to_id:
                id2[i] = elem_to_id[e]
            else:
                elem_to_id[e] = count
                id2[i] = count
                count += 1
        a = OrDistInt(id1, id2)
        id_flg = True
    if mode == "spearman_dist":
        return a.spearman_dist()
    elif mode == "footrule_dist":
        return a.footrule_dist()
    elif mode == "kendall_dist":
        return a.kendall_dist()
    elif mode == "cayley_dist":
        return a.cayley_dist()
    elif mode == "levenshtein_dist":
        return a.levenshtein_dist()
    elif mode == "ulam_dist":
        return a.ulam_dist()
    elif mode == "spearman_corrcoef":
        return a.spearman_corrcoef()
    elif mode == "kendall_corrcoef":
        return a.kendall_corrcoef()
    elif mode == "lcs":
        return a.lcs()
    elif mode == "lcs_with_list":
        n_lcs, l_lcs = a.lcs_with_list()
        if id_flg:
            return n_lcs, [id_to_elem[i] for i in l_lcs]
        else:
            return n_lcs, l_lcs
    else:
        raise ValueError("Unknown mode!")

def spearman_dist(v1, v2):
    return parser(v1, v2, "spearman_dist")
def footrule_dist(v1, v2):
    return parser(v1, v2, "footrule_dist")
def kendall_dist(v1, v2):
    return parser(v1, v2, "kendall_dist")
def cayley_dist(v1, v2):
    return parser(v1, v2, "cayley_dist")
def levenshtein_dist(v1, v2):
    return parser(v1, v2, "levenshtein_dist")
def ulam_dist(v1, v2):
    return parser(v1, v2, "ulam_dist")
def spearman_corrcoef(v1, v2):
    return parser(v1, v2, "spearman_corrcoef")
def kendall_corrcoef(v1, v2):
    return parser(v1, v2, "kendall_corrcoef")
def lcs(v1, v2):
    return parser(v1, v2, "lcs")
def lcs_with_list(v1, v2):
    return parser(v1, v2, "lcs_with_list")