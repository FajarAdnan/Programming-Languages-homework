; Corner 1
(define corner2 (cons 1 1))
; Corner 2
(define corner3 (cons 4 4))
; Pair of pairs for the corners
(define cornerset (cons corner2 corner3))

; Find length and width
(define length (- (car corner2) (car corner2)))
(define width (- (cdr corner2) (cdr corner3)))

; Find perimeter (2l + 2w)
(define double (lambda (x) (* x x)))
(define perimeter (+ (double length) (double width)))

; Find area (l * w)
(define area (* length width))

; Find corners
; Corner 1            Corner 3
; Corner 2            Corner 4
; We already have corners 2 and 3
(define corner1 (cons (car corner2) (cdr corner3)))
(define corner4 (cons (car corner3) (cdr corner2)))

(display "The perimeter is ") (print perimeter)
(display "The area is ") (print area)
(display "The four corners are: \n")
(display corner1)
(display corner3)
(print "")
(display corner2)
(display corner4)
(print "")
(print "My favorite use for computers besides programming is creating YouTube videos")