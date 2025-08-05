(ns difference-of-squares)

(defn square-of-sum
  "Returns the square of the sum of the numbers up to the given number"
  [n]
  (int
     (Math/pow
        (reduce + (range 1 (+ n 1)))
        2))
  )

(defn sum-of-squares
  "Returns the sum of the squares of the numbers up to the given number"
  [n]
  (reduce
     (fn [acc x]
       (+ 
          (int (Math/pow x 2))
          acc)
       )
     (range 1 (+ n 1))
   )
  )

(defn difference
  "Returns the difference between the square of the sum of numbers up to a given number and the sum of the squares of those numbers"
  [n]
  (- (square-of-sum n) (sum-of-squares n))
  )
