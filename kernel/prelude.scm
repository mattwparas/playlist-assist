(require-builtin steel/spotify as spotify.)
(require-builtin steel/random as rand::)

(define (playlist #:name name #:description (description #f) #:archive-to (archive-to #f) #:spec spec)
  (spotify.playlist-creation name description archive-to spec))

(define (song name #:album (album #f) #:artist (artist #f))
  (spotify.track-search-criteria name album artist))

(define (album name #:artist (artist #f) #:spotify-uri (uri #f) #:tracks (tracks (list)))
  (spotify.album-search-criteria name artist uri tracks))

;; Utilities for shuffling the playlist - this will perform a top level shuffle
;; without any kind of additions

(define rng (rand::thread-rng!))
(define (random n)
  (rand::rng->gen-range rng 0 n))

(define rng (rand::thread-rng!))
(define (random n)
  (rand::rng->gen-range rng 0 n))

(define (shuffle list)
  (if (< (length list) 2)
      list
      (let ([item (list-ref list (random (length list)))]) (cons item (shuffle (remove item list))))))

(define (remove item list)
  (cond
    [(empty? list) '()]
    [(equal? item (car list)) (cdr list)]
    [else (cons (car list) (remove item (cdr list)))]))
