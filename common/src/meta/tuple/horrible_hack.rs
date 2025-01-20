macro_rules! index_tuple {
    ($self:expr, $a:ident) => {
        $self.0
    };
    ($self:expr, $a:ident, $b:ident) => {
        $self.1
    };
    ($self:expr, $a:ident, $b:ident, $c:ident) => {
        $self.2
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident) => {
        $self.3
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident) => {
        $self.4
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident) => {
        $self.5
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident) => {
        $self.6
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident) => {
        $self.7
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident) => {
        $self.8
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident) => {
        $self.9
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident) => {
        $self.10
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident) => {
        $self.11
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident) => {
        $self.12
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident) => {
        $self.13
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident) => {
        $self.14
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident) => {
        $self.15
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident) => {
        $self.16
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident) => {
        $self.17
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident) => {
        $self.18
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident) => {
        $self.19
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident) => {
        $self.20
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident) => {
        $self.21
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident, $w:ident) => {
        $self.22
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident, $w:ident, $x:ident) => {
        $self.23
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident, $w:ident, $x:ident, $y:ident) => {
        $self.24
    };
    ($self:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $i:ident, $j:ident, $k:ident, $l:ident, $m:ident, $n:ident, $o:ident, $p:ident, $q:ident, $r:ident, $s:ident, $t:ident, $u:ident, $v:ident, $w:ident, $x:ident, $y:ident, $z:ident) => {
        $self.25
    };
}
