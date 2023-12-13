(begin
    (display "Hello, World!")
    (newline))

; TESTING

(define xvalues (list 10 11 12 14 9))
(define yvalues (list 21 15 23 27 18))

; TESTING

;(define (read-csv file header column))

;(define (regressiona xvalues yvalues))

;(define (regressionb xvalues yvalues))

;(define (correlation xvalues yvalues))

; This function calculates the mean of the values in the list values. 
(define (mean values)
    (let ((sum 0.0) (num 0))
        (define (recursive list)        ; recursive helper function to process through the list 'values'
            (if (null? list)            ; check if the list is empty
                (if (= num 0)           ; Check for divide by zero
                    0                   ; Return 0 - this will be on an empty list
                    (/ sum num)        ; else return 'sum' / 'num'
                )
                (begin
                    (set! sum (+ sum (car list)))   ; at the fist number from the list to 'sum'
                    (set! num (+ num 1))            ; increment 'num' by 1
                    (recursive (cdr list))          ; recursvely call function again passing the remainder of the list using 'cdr'
                )
            )
        )
        (recursive values)                          ; the mean function returns the value returned by the 'recursive' function
    )                 
)

; This function calculates the standard deviation of the values of the list values;
(define (stddev values)
    (let ((avg (mean values)) (num 0) (sum 0))
        (define (recursive list)
            (if (null? list)                ; check if the list is empty
                (if (= num 0)               ; Check for divide by zero
                    0                       ; Return 0 - this will be on an empty list
                    (sqrt (/ sum num))      ; returns the standard deviation
                )
                (begin 
                    (set! sum (+ sum (* (- (car list) avg) (- (car list) avg))))        ; computing numerator from stddev formula
                    (set! num (+ num 1))                                                ; increment 'num' by 1
                    (recursive (cdr list))                                              ; recursvely call function again passing the remainder of the list using 'cdr'
                )
            )
        )
        (recursive values)
    )
)
