(playlist
 #:name "Matt's playlist of the week"
 #:description "My favorite live performances"
 #:archive-to "Matt's playlist of the week: 9/2/2023"
 #:spec
 (cons
  ;; Classic album, the studio versions are really bad relative to this live one
  (album "At Budokan"
         #:artist "Cheap Trick"
         #:tracks (list "Ain't That a Shame" "I Want You to Want Me" "Surrender"))
  (shuffle
   ;; Great album, unfortunately not all of the songs are playable. Hoping that eventually spotify
   ;; retains the rights for them in the future
   (list (album "Alchemy: Dire Straits Live (Remastered)"
                #:artist "Dire Straits"
                ;; This playlist has difficulty showing up!
                #:spotify-uri "spotify:album:0S8mVxRM9uftA1C6dLp7ip")
         ;; My favorite album of all time, the in the cage medley is excellent
         (album "Three Sides Live (1994 Remaster)" #:artist "Genesis")
         (album "Seconds Out (Live)" #:artist "Genesis")
         (album "Frampton Comes Alive! (Deluxe Edition)"
                #:artist "Peter Frampton"
                #:tracks '("Baby, I Love Your Way" "Do You Feel Like We Do - Live"))
         (album "Exit... Stage Left" #:artist "Rush")
         (album "Queen At Live Aid" #:artist "Queen")
         (album "Pulse (Live)" #:artist "Pink Floyd")
         (album "One Night Only: Live Greatest Hits"
                #:artist "Elton John"
                #:tracks (list "I'm Still Standing"))
         (album "The Best of Rock and Roll Hall of Fame + Museum Live")
         (album "A Farewell to Kings (40th Anniversary/Deluxe Edition)" #:artist "Rush")
         (album "Stop Making Sense (Live)" #:artist "Talking Heads" #:tracks (list "Psycho Killer"))
         (song "Jersey Girl - Live at Meadowlands Arena, E. Rutherford, NJ - July 1981")))))
