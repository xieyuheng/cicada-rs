"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
class Top {
    constructor() {
        this.abstract_class_name = "Top";
    }
}
exports.Top = Top;
class TopNamedScopeEntry extends Top {
    constructor(name, entry) {
        super();
        this.name = name;
        this.entry = entry;
    }
}
exports.TopNamedScopeEntry = TopNamedScopeEntry;
class TopKeywordRefuse extends Top {
    constructor(exp, t) {
        super();
        this.exp = exp;
        this.t = t;
    }
}
exports.TopKeywordRefuse = TopKeywordRefuse;
class TopKeywordAccept extends Top {
    constructor(exp, t) {
        super();
        this.exp = exp;
        this.t = t;
    }
}
exports.TopKeywordAccept = TopKeywordAccept;
class TopKeywordShow extends Top {
    constructor(exp) {
        super();
        this.exp = exp;
    }
}
exports.TopKeywordShow = TopKeywordShow;
class TopKeywordEq extends Top {
    constructor(lhs, rhs) {
        super();
        this.lhs = lhs;
        this.rhs = rhs;
    }
}
exports.TopKeywordEq = TopKeywordEq;