import React from 'react';

interface Props {
    name: string;
    age?: number;
}

const UserCard: React.FC<Props> = ({ name, age }) => {
    return (
        <div className="user-card">
            <h2>{name}</h2>
            {age && <p>Age: {age}</p>}
            <button onClick={() => alert(`Hello, ${name}!`)}>
                Greet
            </button>
        </div>
    );
};

export default UserCard;