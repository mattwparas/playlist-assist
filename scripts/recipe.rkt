; *thread-queue* : list[continuation]
(define *thread-queue* '())

; halt : continuation
(define halt #f)

; current-continuation : -> continuation
(define (current-continuation)
  (call/cc
   (lambda (cc)
     (cc cc))))

; await : future -> value
; yield the current thread and loop until the value is completed
(define/contract (await future)
  (->/c future? any/c)
  (define (loop future)
    (define output (poll! future))
    (if (equal? #f output)
        (begin
            (yield)
            (loop future))
        output))
  (loop future))

; spawn : (-> anything) -> void
(define (spawn thunk)
  (let ((cc (current-continuation)))
    (if (continuation? cc)
        (set! *thread-queue* (append *thread-queue* (list cc)))
        (begin 
               (thunk)
               (quit)))))

; yield : value -> void
(define (yield)
  (let ((cc (current-continuation)))
    (if (and (continuation? cc) (pair? *thread-queue*))
        (let ((next-thread (car *thread-queue*)))
          (set! *thread-queue* (append (cdr *thread-queue*) (list cc)))
          (next-thread 'resume))
        void)))

; quit : -> ...
(define (quit)
  (if (pair? *thread-queue*)
      (let ((next-thread (car *thread-queue*)))
        (set! *thread-queue* (cdr *thread-queue*))
        (next-thread 'resume))
      (halt)))
   
; start-threads : -> ...
(define (start-threads)
  (let ((cc (current-continuation)))
    (if cc
        (begin
          (set! halt (lambda () (cc #f)))
          (if (null? *thread-queue*)
              void
              (let ((next-thread (car *thread-queue*)))
                    (set! *thread-queue* (cdr *thread-queue*))
                    (next-thread 'resume))))
        void)))

;; Define an entry point 
(define-syntax async-main
  (syntax-rules ()
    [(async-main main-func)
    (begin main-func (spawn main) (start-threads))]))

;; Use a macro to make it a little bit prettier
; (async-main
;   (define (main)
;     (define test-recipe (await (playlist->recipe *spotify* "DM")))
;     (displayln test-recipe)
;     (register-group 
;         test-recipe
;         '(
;             "Better Not (feat. Wafia)"
;             "Waikiki - Original Mix"
;             "Midsummer Madness"
;         )
;     )

;     ; (register-group
;     ;     test-recipe
;     ;     '(
;     ;         "Blessed By A Nightmare"
;     ;         "Make A Sound"
;     ;     )
;     ; )

;     ; (register-group
;     ;     test-recipe
;     ;     '(
;     ;         "Ocean Avenue"
;     ;         "Rotisserie"
;     ;         "Boomin"
;     ;     )
;     ; )

;     (define output (shuffle test-recipe))
;     (await 
;       (tracklist->playlist *spotify* "rust test playlist 2" output))))


(define/contract (build-recipe recipe)
  (->/c Recipe? void?)
  (register-group 
    recipe
    '(
        "Better Not (feat. Wafia)"
        "Waikiki - Original Mix"
        "Midsummer Madness"
    ))
  (register-group
    recipe
    '(
        "Blessed By A Nightmare"
        "Make A Sound"
    )
  )
  (register-group
      recipe
      '(
          "Ocean Avenue"
          "Rotisserie"
          "Boomin"
       )
  ))


(async-main
  (define (main)
    (let ((test-recipe (await (playlist->recipe *spotify* "DM"))))
      (build-recipe test-recipe)
      (await 
        (tracklist->playlist *spotify* "rust test playlist 2" (shuffle test-recipe))))))
