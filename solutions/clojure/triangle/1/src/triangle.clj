(ns triangle)

(defn is_triangle?
  [a b c]
  (and
     (every? pos? [a b c])
     (and
        (>= (+ a b) c)
        (>= (+ b c) a)
        (>= (+ a c) b)
      )
   )
  )

(defn equilateral?
  "Returns true if the triangle with sides a, b, and c is equilateral; otherwise, returns false"
  [a b c]
  (if (is_triangle? a b c)
    (= a b c)
    false
    )
  )

(defn isosceles?
  "Returns true if the triangle with sides a, b, and c is isosceles; otherwise, returns false"
  [a b c]
    (if (is_triangle? a b c)
      (or (= a b) (= a c) (= b c))
      false
    )
  )

(defn scalene?
  "Returns true if the triangle with sides a, b, and c is scalene; otherwise, returns false"
  [a b c]
    (if (is_triangle? a b c)
      (and (not= a b) (not= a c) (not= b c))
      false
    )
  )
