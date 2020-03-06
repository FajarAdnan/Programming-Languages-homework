;(define list-of-numbers (list 1 2 3 4 5 6 7))
(define list-of-numbers (list 1 (list 2 (list 3 (list 4 (list 5 (list 6 7)))))))

; Print the list that we made
(display "The list is: ")
(display list-of-numbers)
(display "\n")

; Get the length of the list (used for recursion but I couldn't get it to work soooooo)
;(define list-length (length list-of-numbers))
;(display "The length of the list is: \n")
;(display list-length)
;(display "\n")

; Get the final value of the list and print (in this case 7)
; I tried to use recursion so it would work no matter the size of the list but I could not figure it out...
(display "The final value in the list is: ")
(define get-last (car (cdr (car (cdr (car (cdr (car (cdr (car (cdr (car (cdr list-of-numbers)))))))))))))
(display get-last)
(display "\n")

(print "My favorite blog is Geeks for Geeks (I know it's not exactly a blog but holy cow they're helpful")
