
SELECT users.age as age,
       21 as minimum_age,
       'whazzup' as somestuff
   FROM users
   WHERE id=?
