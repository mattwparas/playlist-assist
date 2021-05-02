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

; block-on : future -> value
; blocks the current execution until the given future is complete
(define/contract (block-on future)
  (->/c future? any/c)
  (define (loop future)
    (define output (poll! future))
    (if (equal? #f output)
        ;; Here, we would yield to the executor if we were await-ing
        ;; but we're going to just block until the future is complete
        (loop future)
        output))
  (loop future))

(define (playlist->recipe playlist-name)
  (block-on (async-playlist->recipe *spotify* playlist-name)))

(define (tracklist->playlist playlist-name tracklist)
  (block-on (async-tracklist->playlist *spotify* playlist-name tracklist)))


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

(define (main)
  (let ((test-recipe (playlist->recipe "DM")))
    (build-recipe test-recipe)
    (tracklist->playlist "rust test playlist 2" (shuffle test-recipe))))

(main)
